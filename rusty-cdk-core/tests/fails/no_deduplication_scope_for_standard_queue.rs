use rusty_cdk_core::sqs::{QueueBuilder, DeduplicationScope};
use rusty_cdk_core::wrappers::StringWithOnlyAlphaNumericsAndUnderscores;
use rusty_cdk_macros::string_with_only_alphanumerics_and_underscores;
use rusty_cdk_core::stack::StackBuilder;

fn example() {
    let mut stack_builder = StackBuilder::new();
    let queue_name = string_with_only_alphanumerics_and_underscores!("test_queue");
    QueueBuilder::new("myQueue")
        .standard_queue()
        .queue_name(queue_name)
        .deduplication_scope(DeduplicationScope::Queue)
        .build(&mut stack_builder);
}

fn main() {}
