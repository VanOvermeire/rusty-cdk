use serde_json::{json, Value};

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

pub fn join(delimiter: &str, elements: Vec<Value>) -> Value {
    json!({
        "Fn::Join": [
            delimiter,
            elements
        ]
    })
}
