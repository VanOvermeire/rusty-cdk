use serde::Serialize;
use serde_json::Value;
use crate::intrinsic_functions::get_ref;

#[derive(Debug, Serialize)]
pub struct LogGroup {
    #[serde(skip)]
    pub(crate) id: String,
    #[serde(rename = "Type")]
    pub(crate) r#type: String,
    #[serde(rename = "Properties")]
    pub(crate) properties: LogGroupProperties,
}

impl LogGroup {
    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }
    
    pub fn get_ref(&self) -> Value {
        get_ref(self.get_id())
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
