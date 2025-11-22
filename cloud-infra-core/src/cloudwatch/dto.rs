use serde::Serialize;
use serde_json::Value;
use crate::{dto_methods, ref_struct};
use crate::shared::Id;

ref_struct!(LogGroupRef);

#[derive(Debug, Serialize)]
pub struct LogGroup {
    #[serde(skip)]
    pub(super) id: Id,
    #[serde(skip)]
    pub(super) resource_id: String,
    #[serde(rename = "Type")]
    pub(super) r#type: String,
    #[serde(rename = "Properties")]
    pub(super) properties: LogGroupProperties,
}
dto_methods!(LogGroup);

#[derive(Debug, Serialize)]
pub struct LogGroupProperties {
    #[serde(rename = "LogGroupClass", skip_serializing_if = "Option::is_none")]
    pub(super) log_group_class: Option<String>,
    #[serde(rename = "LogGroupName", skip_serializing_if = "Option::is_none")]
    pub(super) log_group_name: Option<Value>,
    #[serde(rename = "RetentionInDays", skip_serializing_if = "Option::is_none")]
    pub(super) log_group_retention: Option<u16>,
}
