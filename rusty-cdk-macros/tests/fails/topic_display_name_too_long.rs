use rusty_cdk_macros::topic_display_name;
struct TopicDisplayName(String);
fn main() {
    topic_display_name!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
}
