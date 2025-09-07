use std::fmt::{Display, Formatter};
use cloud_infra_core::stack::{Asset, Resource, Stack, StackBuilder};

pub struct Synth(pub String, pub(crate) Vec<Asset>);

impl Display for Synth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl TryFrom<Stack> for Synth {
    type Error = String;

    fn try_from(value: Stack) -> Result<Self, Self::Error> {
        let assets = value.get_assets();
        serde_json::to_string(&value)
            .map(|s| Synth(s, assets))
            .map_err(|e| format!("Could not serialize resources: {e:?}"))
    }
}

impl TryFrom<Vec<Resource>> for Synth {
    type Error = String;

    fn try_from(resources: Vec<Resource>) -> Result<Self, Self::Error> {
        let stack_builder = StackBuilder::new().add_resources(resources);
        let stack = stack_builder.build().map_err(|e| e.to_string())?;
        stack.try_into()
    }
}
