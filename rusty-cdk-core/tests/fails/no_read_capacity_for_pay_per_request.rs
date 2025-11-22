use rusty_cdk_core::dynamodb::{AttributeType, Key, TableBuilder};
use rusty_cdk_core::wrappers::StringWithOnlyAlphaNumericsAndUnderscores;
use rusty_cdk_macros::string_with_only_alpha_numerics_and_underscores;
use rusty_cdk_core::stack::StackBuilder;

fn example() {
    let mut stack_builder = StackBuilder::new();
    let key = string_with_only_alpha_numerics_and_underscores!("test");
    TableBuilder::new("myTable", Key::new(key, AttributeType::String))
        .pay_per_request_billing()
        .read_capacity(5)
        .build(&mut stack_builder);
}

fn main() {}
