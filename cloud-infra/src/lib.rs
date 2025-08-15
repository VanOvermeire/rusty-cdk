use std::fmt::{Display, Formatter};
use aws_sdk_cloudformation::types::Capability;
use cloud_infra_core::stack::{Resource, Stack};

pub struct Synth(pub String);

impl Display for Synth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

pub fn synth(resources: Vec<Resource>) -> Result<Synth, String> {
    let stack = Stack::new(resources);
    serde_json::to_string(&stack)
        .map(|s| Synth(s))
        .map_err(|e| format!("Could not serialize resources: {e:?}"))
}

pub async fn deploy(name: &str, synth: Synth) {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_cloudformation::Client::new(&config);
    match client.create_stack()
        .stack_name(name)
        .template_body(synth.0)
        .capabilities(Capability::CapabilityNamedIam)
        .send().await {
        Ok(_) => println!("Stack {name} creation started"),
        Err(e) => eprintln!("An error occurred while creating the stack: {e:?}")
    }
}
