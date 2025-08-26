use std::fmt::{Display, Formatter};
use cloud_infra_core::stack::{Asset, Resource, Stack, StackBuilder};

pub struct Synth(pub String, pub(crate) Vec<Asset>);

impl Display for Synth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

pub fn synth(resources: Vec<Resource>) -> Result<Synth, String> {
    let mut stack_builder = StackBuilder::new();
    resources.into_iter().for_each(|r| stack_builder.add_resource(r));
    let stack = stack_builder.build().map_err(|e| e.to_string())?;
    let assets = stack.get_assets();

    serde_json::to_string(&stack)
        .map(|s| Synth(s, assets))
        .map_err(|e| format!("Could not serialize resources: {e:?}"))
}

pub fn synth_stack(stack: Stack) -> Result<Synth, String> {
    let assets = stack.get_assets();

    serde_json::to_string(&stack)
        .map(|s| Synth(s, assets))
        .map_err(|e| format!("Could not serialize resources: {e:?}"))
}
