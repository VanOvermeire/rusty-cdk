use aws_sdk_cloudcontrol::Client;
use aws_sdk_cloudcontrol::types::ResourceDescription;
use serde::Deserialize;

#[derive(Debug)]
pub(crate) struct ResourceInfo {
    pub(crate) identifier: String,
}

#[derive(Debug)]
pub(crate) struct ResourceInfoWithArn {
    pub(crate) identifier: String,
    pub(crate) arn: String,
}

#[derive(Deserialize, Debug)]
struct InternalResourceDescription {
    #[serde(rename = "Arn")]
    arn: Option<String>,
}

pub(crate) struct CloudControlClient {
    client: Client,
}

impl CloudControlClient {
    pub(crate) async fn new() -> Self {
        let config = aws_config::load_from_env().await;
        CloudControlClient {
            client: Client::new(&config),
        }
    }

    async fn get_internal_resource(&self, identifier: &str, type_name: &str) -> Result<(String, InternalResourceDescription), String> {
        let result = self
            .client
            .get_resource()
            .identifier(identifier)
            .type_name(type_name)
            .send()
            .await
            .map_err(|e| format!("could not retrieve {type_name}: {e}"))?;

        match result.resource_description {
            Some(ResourceDescription {
                identifier: Some(id),
                properties: Some(props),
                ..
            }) => {
                let descr = serde_json::from_str(&props).map_err(|_| "could not read resource info")?;
                Ok((id, descr))
            }
            _ => Err("missing required resource info".to_string()),
        }
    }

    pub(crate) async fn get_resource_arn(&self, identifier: &str, type_name: &str) -> Result<ResourceInfoWithArn, String> {
        let (identifier, internal) = self.get_internal_resource(identifier, type_name).await?;

        if let InternalResourceDescription { arn: Some(arn), .. } = internal {
            Ok(ResourceInfoWithArn { identifier, arn })
        } else {
            Err("missing required resource info: arn".to_string())
        }
    }

    pub(crate) async fn get_resource(&self, identifier: &str, type_name: &str) -> Result<ResourceInfo, String> {
        let (identifier, _internal) = self.get_internal_resource(identifier, type_name).await?;

        Ok(ResourceInfo { identifier })
    }
}

pub(crate) async fn lookup_arn(identifier: &str, type_name: &str) -> Result<ResourceInfoWithArn, String> {
    let client = CloudControlClient::new().await;
    client.get_resource_arn(identifier, type_name).await
}

pub(crate) async fn lookup(identifier: &str, type_name: &str) -> Result<ResourceInfo, String> {
    let client = CloudControlClient::new().await;
    client.get_resource(identifier, type_name).await
}
