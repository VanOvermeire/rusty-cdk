use cloud_infra_core::stack::{Resource, Stack};

pub fn synth(resources: Vec<Resource>) -> Result<String, String> {
    let stack = Stack::new(resources);
    serde_json::to_string(&stack).map_err(|e| format!("Could not serialize resources: {e}"))
}

pub fn deploy() {
       
}
