use cloud_infra_core::sqs::{QueueBuilder, DeduplicationScope};
use cloud_infra_core::wrappers::StringWithOnlyAlphaNumericsAndUnderscores;
use cloud_infra_macros::string_with_only_alpha_numerics_and_underscores;
use cloud_infra_core::stack::StackBuilder;

fn example() {
    let mut stack_builder = StackBuilder::new();
    let queue_name = string_with_only_alpha_numerics_and_underscores!("test_queue");
    QueueBuilder::new("myQueue")
        .standard_queue()
        .queue_name(queue_name)
        .deduplication_scope(DeduplicationScope::Queue)
        .build(&mut stack_builder);
}

fn main() {}
