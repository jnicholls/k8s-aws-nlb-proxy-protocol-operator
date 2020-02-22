#[macro_use]
extern crate log;

use std::{env, error::Error as StdError, time::Duration};

use k8s_openapi::api::core::v1::{ServiceSpec, ServiceStatus};
use kube::{
    api::{Api, Object, Reflector},
    client::APIClient,
    config,
};
use rusoto_core::Region;
use rusoto_elbv2::{
    DescribeTargetGroupAttributesInput, Elb, ElbClient, ModifyTargetGroupAttributesInput,
    TargetGroupAttribute,
};
use rusoto_resourcegroupstaggingapi::{
    GetResourcesInput, ResourceGroupsTaggingApi, ResourceGroupsTaggingApiClient, TagFilter,
};
use serde_json::json;

type Service = Object<ServiceSpec, ServiceStatus>;

#[tokio::main(core_threads = 1)]
async fn main() -> Result<(), Box<dyn StdError>> {
    env_logger::init();

    let namespace = env::var("NAMESPACE").unwrap_or("default".into());
    info!("Scanning for services in namespace '{}'.", namespace);

    let k8s_client = load_api_client(&namespace).await?;
    let rf = Reflector::new(k8s_client.clone()).init().await?;

    tokio::spawn(poll_services(rf.clone()));
    loop {
        match scan_services(&namespace, &k8s_client, &rf).await {
            Err(e) => error!(
                "An error occurred while scanning or processing services: {}.",
                e
            ),
            _ => (),
        }
        tokio::time::delay_for(Duration::from_secs(10)).await;
    }
}

async fn load_api_client(namespace: &str) -> Result<Api<Service>, Box<dyn StdError>> {
    let config = if env::var("KUBERNETES_SERVICE_HOST").is_ok()
        && env::var("KUBERNETES_SERVICE_PORT").is_ok()
    {
        config::incluster_config()
    } else {
        config::load_kube_config().await
    }?;

    // Setup the kube-api Reflector.
    let client = APIClient::new(config);
    Ok(Api::v1Service(client).within(namespace))
}

async fn poll_services(rf: Reflector<Service>) {
    loop {
        match rf.poll().await {
            Err(e) => error!("Failed to poll services: {}.", e),
            _ => (),
        }
    }
}

async fn scan_services(
    namespace: &str,
    k8s_client: &Api<Service>,
    rf: &Reflector<Service>,
) -> Result<(), Box<dyn StdError>> {
    let region = Region::default();
    let elb_client = ElbClient::new(region.clone());
    let tags_client = ResourceGroupsTaggingApiClient::new(region);

    let services = rf.state().await?.into_iter().filter(is_matching_service);

    for service in services {
        info!(
            "Turning on PROXY protocol for the service '{}'.",
            service.metadata.name
        );

        let input = GetResourcesInput {
            resource_type_filters: Some(vec!["elasticloadbalancing:targetgroup".to_string()]),
            tag_filters: Some(vec![TagFilter {
                key: Some("kubernetes.io/service-name".to_string()),
                values: Some(vec![format!("{}/{}", namespace, service.metadata.name)]),
            }]),
            ..Default::default()
        };
        let output = tags_client.get_resources(input).await?;

        let target_group_arns = output
            .resource_tag_mapping_list
            .map(|rtml| {
                rtml.into_iter()
                    .filter_map(|rtm| rtm.resource_arn)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        if target_group_arns.is_empty() {
            warn!(
                "Did not find any TargetGroups for the service '{}'.",
                service.metadata.name
            );
        }

        for target_group_arn in target_group_arns {
            let input = DescribeTargetGroupAttributesInput {
                target_group_arn: target_group_arn.clone(),
                ..Default::default()
            };
            let output = elb_client.describe_target_group_attributes(input).await?;

            let proxy_disabled = output
                .attributes
                .map(|attributes| {
                    attributes.into_iter().any(|attribute| {
                        attribute.key.as_ref().map(|k| k.as_str())
                            == Some("proxy_protocol_v2.enabled")
                            && attribute.value.as_ref().map(|k| k.as_str()) == Some("false")
                    })
                })
                .unwrap_or_default();

            if proxy_disabled {
                let input = ModifyTargetGroupAttributesInput {
                    attributes: vec![TargetGroupAttribute {
                        key: Some("proxy_protocol_v2.enabled".to_string()),
                        value: Some("true".to_string()),
                    }],
                    target_group_arn,
                };
                elb_client.modify_target_group_attributes(input).await?;

                info!(
                    "Service '{}' now has PROXY protocol enabled.",
                    service.metadata.name
                );
            }

            let patch = json!({
                "metadata": {
                    "annotations": {
                        "ironnet.com/k8s-aws-nlb-proxy-protocol-operator": "*"
                    }
                }
            });
            k8s_client
                .patch(
                    &service.metadata.name,
                    &Default::default(),
                    serde_json::to_vec(&patch)?,
                )
                .await?;
            info!(
                "Service '{}' attributes have been patched.",
                service.metadata.name
            );
        }
    }

    Ok(())
}

fn is_matching_service(service: &Service) -> bool {
    let annotations = &service.metadata.annotations;
    let lb_type = annotations
        .get("service.beta.kubernetes.io/aws-load-balancer-type")
        .map(AsRef::as_ref);
    let lb_proxy = annotations
        .get("service.beta.kubernetes.io/aws-load-balancer-proxy-protocol")
        .map(AsRef::as_ref);
    let op_handled = annotations
        .get("ironnet.com/k8s-aws-nlb-proxy-protocol-operator")
        .map(AsRef::as_ref);

    match (lb_type, lb_proxy, op_handled) {
        (Some("nlb"), Some("*"), Some("*")) => false,
        (Some("nlb"), Some("*"), _) => true,
        _ => false,
    }
}
