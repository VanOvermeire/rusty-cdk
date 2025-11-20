use cloud_infra_core::dynamodb::{AttributeType, Key, TableBuilder};
use cloud_infra_core::wrappers::StringWithOnlyAlphaNumericsAndUnderscores;
use cloud_infra_macros::string_with_only_alpha_numerics_and_underscores;
use cloud_infra_core::stack::StackBuilder;

fn example() {
    let mut stack_builder = StackBuilder::new();
    let key = string_with_only_alpha_numerics_and_underscores!("test");
    TableBuilder::new("myTable", Key::new(key, AttributeType::String))
        .pay_per_request_billing()
        .read_capacity(5)
        .build(&mut stack_builder);
}

fn main() {}
