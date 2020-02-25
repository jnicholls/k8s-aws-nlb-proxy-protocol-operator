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
    DescribeTargetGroupAttributesInput, Elb, ElbClient, ModifyTargetGroupAttributesInput, TargetGroupAttribute,
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

    // Start a WATCH call on all services and continually receive updates on any and all
    // changes.
    tokio::spawn(poll_services(rf.clone()));

    // Every 10 seconds, scan for services that are configured with NLB + PROXY protocol
    // annotations, but have not had their NLB TargetGroups setup with the PROXY protocol
    // enabled yet. After enabling the PROXY protocol, the service is annotated to note
    // that it has been processed successfully.
    // Likewise, services that have been annotated as having PROXY protocol but is missing
    // the official Kubernetes annotation will have the PROXY protocol disabled for all
    // of its TargetGroups.
    loop {
        if let Err(e) = scan_services(&namespace, &k8s_client, &rf).await {
            error!("An error occurred while scanning or processing services: {}.", e);
        }
        tokio::time::delay_for(Duration::from_secs(10)).await;
    }
}

async fn load_api_client(namespace: &str) -> Result<Api<Service>, Box<dyn StdError>> {
    // Load a Kubernetes API client from either in-cluster information or from a kubeconfig
    // file located in the user's profile.
    let config = if env::var("KUBERNETES_SERVICE_HOST").is_ok() && env::var("KUBERNETES_SERVICE_PORT").is_ok() {
        config::incluster_config()
    } else {
        config::load_kube_config().await
    }?;

    let client = APIClient::new(config);
    Ok(Api::v1Service(client).within(namespace))
}

async fn poll_services(rf: Reflector<Service>) {
    loop {
        if let Err(e) = rf.poll().await {
            error!("Failed to poll services: {}.", e);
        }
    }
}

async fn scan_services(
    namespace: &str,
    k8s_client: &Api<Service>,
    rf: &Reflector<Service>,
) -> Result<(), Box<dyn StdError>> {
    // Configure AWS clients for the ELBv2 and ResourceGroupsTagging APIs using automatically discovered
    // environmental information such as environment variables, user profile shared credential file, etc.
    // similarly to how the AWS CLI functions.
    let region = Region::default();
    let elb_client = ElbClient::new(region.clone());
    let tags_client = ResourceGroupsTaggingApiClient::new(region);

    // Find all services that should be inspected to either enable or disable the PROXY protocol.
    let services = rf.state().await?.into_iter().filter_map(is_matching_service);

    for (service, turn_on_proxy) in services {
        info!(
            "Turning {} PROXY protocol for the service '{}'.",
            if turn_on_proxy { "on" } else { "off" },
            service.metadata.name
        );

        // Query for all TargetGroup AWS resources that belong to this service.
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
            .map(|rtml| rtml.into_iter().filter_map(|rtm| rtm.resource_arn).collect::<Vec<_>>())
            .unwrap_or_default();

        if target_group_arns.is_empty() {
            warn!(
                "Did not find any TargetGroups for the service '{}'.",
                service.metadata.name
            );
        }

        for target_group_arn in target_group_arns {
            // Load this TargetGroup's attributes.
            let input = DescribeTargetGroupAttributesInput {
                target_group_arn: target_group_arn.clone(),
                ..Default::default()
            };
            let output = elb_client.describe_target_group_attributes(input).await?;

            // Determine if this TargetGroup has the PROXY protocol disabled or not.
            let proxy_disabled = output
                .attributes
                .map(|attributes| {
                    attributes.into_iter().any(|attribute| {
                        attribute.key.as_ref().map(|k| k.as_str()) == Some("proxy_protocol_v2.enabled")
                            && attribute.value.as_ref().map(|k| k.as_str()) == Some("false")
                    })
                })
                .unwrap_or_default();

            // If the PROXY protocol is currently disabled and the service wants it enabled, then enable it.
            // Or, If the PROXY protocol is currently enabled and the service wants it disabled, then disable it.
            let modify_target_group = match (turn_on_proxy, proxy_disabled) {
                (true, true) => Some(("true", "enabled", "*")),
                (false, false) => Some(("false", "disabled", "")),
                _ => None,
            };
            if let Some((value, status, annotation)) = modify_target_group {
                let input = ModifyTargetGroupAttributesInput {
                    attributes: vec![TargetGroupAttribute {
                        key: Some("proxy_protocol_v2.enabled".to_string()),
                        value: Some(value.to_string()),
                    }],
                    target_group_arn: target_group_arn.clone(),
                };
                elb_client.modify_target_group_attributes(input).await?;

                // Patch the service with our proprietary annotation to mark that the service has had
                // its PROXY protocol enabled successfully.
                let patch = json!({
                    "metadata": {
                        "annotations": {
                            "jarrednicholls.com/k8s-aws-nlb-proxy-protocol-operator": annotation
                        }
                    }
                });
                k8s_client
                    .patch(&service.metadata.name, &Default::default(), serde_json::to_vec(&patch)?)
                    .await?;

                info!(
                    "Target Group '{}' for service '{}' now has PROXY protocol {}.",
                    target_group_arn, service.metadata.name, status
                );
            }
        }
    }

    Ok(())
}

fn is_matching_service(service: Service) -> Option<(Service, bool)> {
    let annotations = &service.metadata.annotations;
    let lb_type = annotations
        .get("service.beta.kubernetes.io/aws-load-balancer-type")
        .map(AsRef::as_ref);
    let lb_proxy = annotations
        .get("service.beta.kubernetes.io/aws-load-balancer-proxy-protocol")
        .map(AsRef::as_ref);
    let op_handled = annotations
        .get("jarrednicholls.com/k8s-aws-nlb-proxy-protocol-operator")
        .map(AsRef::as_ref);

    match (lb_type, lb_proxy, op_handled) {
        // PROXY has already been enabled, so we'll ignore this service.
        (Some("nlb"), Some("*"), Some("*")) => None,

        // PROXY annotations are present but PROXY has not been enabled yet.
        (Some("nlb"), Some("*"), _) => Some((service, true)),

        // PROXY has been enabled but annotations have been removed since, so we will disable PROXY.
        (_, _, Some("*")) => Some((service, false)),

        // Any other results are ignored.
        _ => None,
    }
}
