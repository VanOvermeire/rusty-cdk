use aws_sdk_cloudcontrol::Client;
use aws_sdk_cloudcontrol::types::ResourceDescription;
use serde::Deserialize;

#[derive(Debug)]
pub(crate) struct ResourceInfo {
    pub(crate) identifier: String,
    pub(crate) arn: String
}

#[derive(Deserialize, Debug)]
struct InternalResourceDescription {
    #[serde(rename="Arn")]
    arn: String
}

pub(crate) struct CloudControlClient {
    client: Client
}

impl CloudControlClient {
    pub(crate) async fn new() -> Self {
        let config = aws_config::load_from_env().await;
        CloudControlClient { client: Client::new(&config) }
    }

    pub(crate) async fn get_resource(&self, identifier: &str, type_name: &str) -> Result<ResourceInfo, String> {
        let result = self.client.get_resource()
            .identifier(identifier)
            .type_name(type_name)
            .send()
            .await
            .map_err(|e| format!("could not retrieve {type_name}: {e}"))?;
        
        match result.resource_description {
            Some(ResourceDescription { identifier: Some(id), properties: Some(props) , .. }) => {
                let descr: InternalResourceDescription = serde_json::from_str(&props).map_err(|_| {
                    "could not read resource info"
                })?;
                
                Ok(ResourceInfo {
                    identifier: id,
                    arn: descr.arn,
                })
            },
            _ => {
                Err("missing required resource info".to_string())
            } 
        }
    }
}

pub(crate) async fn lookup(identifier: &str, type_name: &str) -> Result<ResourceInfo, String> {
    let client = CloudControlClient::new().await;
    client.get_resource(identifier, type_name).await
}