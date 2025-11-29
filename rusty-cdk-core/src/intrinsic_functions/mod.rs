// TODO rename mod to intrinsic

use serde_json::{json, Value};

pub const AWS_ACCOUNT_PSEUDO_PARAM: &str = "AWS::AccountId";
pub const AWS_PARTITION_PSEUDO_PARAM: &str = "AWS::Partition";
pub const AWS_REGION_PSEUDO_PARAM: &str = "AWS::Region";

pub fn get_arn(id: &str) -> Value {
    json!({
        "Fn::GetAtt": [
            id,
            "Arn"
        ]
    })
}

pub fn get_ref(id: &str) -> Value {
    json!({
        "Ref": id
    })
}

pub fn get_att(id: &str, attribute: &str) -> Value {
    json!({
        "Fn::GetAtt": [ id, attribute ]
    })
}

pub fn join(delimiter: &str, elements: Vec<Value>) -> Value {
    json!({
        "Fn::Join": [
            delimiter,
            elements
        ]
    })
}
