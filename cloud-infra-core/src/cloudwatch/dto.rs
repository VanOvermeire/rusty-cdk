use serde::Serialize;
use serde_json::Value;
use crate::ref_struct;
use crate::shared::Id;

ref_struct!(LogGroupRef);

#[derive(Debug, Serialize)]
pub struct LogGroup {
    #[serde(skip)]
    pub(crate) id: Id,
    #[serde(skip)]
    pub(crate) resource_id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: LogGroupProperties,
}

impl LogGroup {
    pub fn get_id(&self) -> &Id {
        &self.id
    }
    
    pub fn get_resource_id(&self) -> &str {
        self.resource_id.as_str()
    }
}

#[derive(Debug, Serialize)]
pub struct LogGroupProperties {
    #[serde(rename = "LogGroupClass", skip_serializing_if = "Option::is_none")]
    pub(crate) log_group_class: Option<String>,
    #[serde(rename = "LogGroupName", skip_serializing_if = "Option::is_none")]
    pub(crate) log_group_name: Option<Value>,
    #[serde(rename = "RetentionInDays", skip_serializing_if = "Option::is_none")]
    pub(crate) log_group_retention: Option<u16>,
}
