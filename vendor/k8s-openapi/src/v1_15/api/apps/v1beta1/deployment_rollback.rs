// Generated from definition io.k8s.api.apps.v1beta1.DeploymentRollback

/// DEPRECATED. DeploymentRollback stores the information required to rollback a deployment.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeploymentRollback {
    /// Required: This must match the Name of a deployment.
    pub name: String,

    /// The config of this deployment rollback.
    pub rollback_to: crate::api::apps::v1beta1::RollbackConfig,

    /// The annotations to be updated to a deployment
    pub updated_annotations: Option<std::collections::BTreeMap<String, String>>,
}

// Begin apps/v1beta1/DeploymentRollback

// Generated from operation createAppsV1beta1NamespacedDeploymentRollback

impl DeploymentRollback {
    /// create rollback of a Deployment
    ///
    /// Use the returned [`crate::ResponseBody`]`<`[`CreateNamespacedDeploymentRollbackResponse`]`>` constructor, or [`CreateNamespacedDeploymentRollbackResponse`] directly, to parse the HTTP response.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    ///     name of the DeploymentRollback
    ///
    /// * `namespace`
    ///
    ///     object name and auth scope, such as for teams and projects
    ///
    /// * `body`
    ///
    /// * `optional`
    ///
    ///     Optional parameters. Use `Default::default()` to not pass any.
    #[cfg(feature = "api")]
    pub fn create_namespaced_deployment_rollback(
        name: &str,
        namespace: &str,
        body: &crate::api::apps::v1beta1::DeploymentRollback,
        optional: crate::CreateOptional<'_>,
    ) -> Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> crate::ResponseBody<CreateNamespacedDeploymentRollbackResponse>), crate::RequestError> {
        let __url = format!("/apis/apps/v1beta1/namespaces/{namespace}/deployments/{name}/rollback?",
            name = crate::percent_encoding::percent_encode(name.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
            namespace = crate::percent_encoding::percent_encode(namespace.as_bytes(), crate::percent_encoding2::PATH_SEGMENT_ENCODE_SET),
        );
        let mut __query_pairs = crate::url::form_urlencoded::Serializer::new(__url);
        optional.__serialize(&mut __query_pairs);
        let __url = __query_pairs.finish();

        let __request = http::Request::post(__url);
        let __body = serde_json::to_vec(body).map_err(crate::RequestError::Json)?;
        let __request = __request.header(http::header::CONTENT_TYPE, http::header::HeaderValue::from_static("application/json"));
        match __request.body(__body) {
            Ok(request) => Ok((request, crate::ResponseBody::new)),
            Err(err) => Err(crate::RequestError::Http(err)),
        }
    }
}

/// Use `<CreateNamespacedDeploymentRollbackResponse as Response>::try_from_parts` to parse the HTTP response body of [`DeploymentRollback::create_namespaced_deployment_rollback`]
#[cfg(feature = "api")]
#[derive(Debug)]
pub enum CreateNamespacedDeploymentRollbackResponse {
    Ok(crate::apimachinery::pkg::apis::meta::v1::Status),
    Created(crate::apimachinery::pkg::apis::meta::v1::Status),
    Accepted(crate::apimachinery::pkg::apis::meta::v1::Status),
    Other(Result<Option<serde_json::Value>, serde_json::Error>),
}

#[cfg(feature = "api")]
impl crate::Response for CreateNamespacedDeploymentRollbackResponse {
    fn try_from_parts(status_code: http::StatusCode, buf: &[u8]) -> Result<(Self, usize), crate::ResponseError> {
        match status_code {
            http::StatusCode::OK => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedDeploymentRollbackResponse::Ok(result), buf.len()))
            },
            http::StatusCode::CREATED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedDeploymentRollbackResponse::Created(result), buf.len()))
            },
            http::StatusCode::ACCEPTED => {
                let result = match serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                    Err(err) => return Err(crate::ResponseError::Json(err)),
                };
                Ok((CreateNamespacedDeploymentRollbackResponse::Accepted(result), buf.len()))
            },
            _ => {
                let (result, read) =
                    if buf.is_empty() {
                        (Ok(None), 0)
                    }
                    else {
                        match serde_json::from_slice(buf) {
                            Ok(value) => (Ok(Some(value)), buf.len()),
                            Err(ref err) if err.is_eof() => return Err(crate::ResponseError::NeedMoreData),
                            Err(err) => (Err(err), 0),
                        }
                    };
                Ok((CreateNamespacedDeploymentRollbackResponse::Other(result), read))
            },
        }
    }
}

// End apps/v1beta1/DeploymentRollback

impl crate::Resource for DeploymentRollback {
    const API_VERSION: &'static str = "apps/v1beta1";
    const GROUP: &'static str = "apps";
    const KIND: &'static str = "DeploymentRollback";
    const VERSION: &'static str = "v1beta1";
}

impl<'de> serde::Deserialize<'de> for DeploymentRollback {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_name,
            Key_rollback_to,
            Key_updated_annotations,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "name" => Field::Key_name,
                            "rollbackTo" => Field::Key_rollback_to,
                            "updatedAnnotations" => Field::Key_updated_annotations,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeploymentRollback;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as crate::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_name: Option<String> = None;
                let mut value_rollback_to: Option<crate::api::apps::v1beta1::RollbackConfig> = None;
                let mut value_updated_annotations: Option<std::collections::BTreeMap<String, String>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::API_VERSION {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::KIND {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::KIND));
                            }
                        },
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_rollback_to => value_rollback_to = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_updated_annotations => value_updated_annotations = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeploymentRollback {
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    rollback_to: value_rollback_to.ok_or_else(|| serde::de::Error::missing_field("rollbackTo"))?,
                    updated_annotations: value_updated_annotations,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as crate::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "name",
                "rollbackTo",
                "updatedAnnotations",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for DeploymentRollback {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as crate::Resource>::KIND,
            4 +
            self.updated_annotations.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "rollbackTo", &self.rollback_to)?;
        if let Some(value) = &self.updated_annotations {
            serde::ser::SerializeStruct::serialize_field(&mut state, "updatedAnnotations", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
