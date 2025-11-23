use rusty_cdk_core::dynamodb::{AttributeType, Key, TableBuilder};
use rusty_cdk_core::wrappers::{StringWithOnlyAlphaNumericsAndUnderscores,NonZeroNumber};
use rusty_cdk_macros::{string_with_only_alphanumerics_and_underscores, non_zero_number};
use rusty_cdk_core::stack::StackBuilder;

fn example() {
    let mut stack_builder = StackBuilder::new();
    let key = string_with_only_alphanumerics_and_underscores!("test");
    let max_read = non_zero_number!(100);
    TableBuilder::new("myTable", Key::new(key, AttributeType::String))
        .provisioned_billing()
        .max_read_capacity(max_read)
        .build(&mut stack_builder);
}

fn main() {}
