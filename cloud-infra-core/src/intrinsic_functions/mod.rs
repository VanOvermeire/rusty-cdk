use serde_json::{json, Value};

pub fn get_arn(id: &str) -> Value {
    json!({
        "Fn::GetAtt": [
            id,
            "Arn"
        ]
    })
}

// TODO generic...
pub fn join() -> Value {
    json!({
        "Fn::Join": [
        "",
        [
            "arn:",
            {
                "Ref": "AWS::Partition"
            },
            ":iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
        ]]
    })
}
