use serde_json::Value;
use crate::cloudwatch::{LogGroup, LogGroupProperties};
use crate::stack::Resource;
use crate::wrappers::{LogGroupName, RetentionInDays};

pub enum LogGroupClass {
    Standard,
    InfrequentAccess,
}

impl From<LogGroupClass> for String {
    fn from(value: LogGroupClass) -> Self {
        match value {
            LogGroupClass::Standard => "STANDARD".to_string(),
            LogGroupClass::InfrequentAccess => "INFREQUENT_ACCESS".to_string(),
        }
    }
}

pub struct LogGroupBuilder {
    log_group_name: Option<Value>,
    log_group_class: Option<LogGroupClass>,
    log_group_retention: Option<u16>
}

impl LogGroupBuilder {
    pub fn new() -> Self {
        Self {
            log_group_name: None,
            log_group_class: None,
            log_group_retention: None,
        }
    }
    
    pub fn log_group_name_string(self, log_group_name: LogGroupName) -> Self {
        Self {
            log_group_name: Some(Value::String(log_group_name.0)),
            ..self
        }
    }
    
    pub fn log_group_name_value(self, log_group_name: Value) -> Self {
        Self {
            log_group_name: Some(log_group_name),
            ..self
        }
    }
    
    pub fn log_group_class(self, log_group_class: LogGroupClass) -> Self {
        Self {
            log_group_class: Some(log_group_class),
            ..self
        }
    }
    
    pub fn log_group_retention(self, log_group_retention_in_days: RetentionInDays) -> Self {
        Self {
            log_group_retention: Some(log_group_retention_in_days.0), 
            ..self
        }
    }
    
    pub fn build(self) -> LogGroup {
        let properties = LogGroupProperties {
            log_group_name: self.log_group_name,
            log_group_class: self.log_group_class.map(Into::into),
            log_group_retention: self.log_group_retention,
        };
        
        LogGroup {
            id: Resource::generate_id("LogGroup"),
            r#type: "AWS::Logs::LogGroup".to_string(),
            properties,
        }
    }
}
