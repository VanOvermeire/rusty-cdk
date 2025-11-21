use cloud_infra_core::dynamodb::{AttributeType, Key, TableBuilder};
use cloud_infra_core::wrappers::{StringWithOnlyAlphaNumericsAndUnderscores,NonZeroNumber};
use cloud_infra_macros::{string_with_only_alpha_numerics_and_underscores, non_zero_number};
use cloud_infra_core::stack::StackBuilder;

fn example() {
    let mut stack_builder = StackBuilder::new();
    let key = string_with_only_alpha_numerics_and_underscores!("test");
    let max_read = non_zero_number!(100);
    TableBuilder::new("myTable", Key::new(key, AttributeType::String))
        .provisioned_billing()
        .max_read_capacity(max_read)
        .build(&mut stack_builder);
}

fn main() {}
