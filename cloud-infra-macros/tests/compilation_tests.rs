#![allow(dead_code)]

use cloud_infra_macros::non_zero_number;

// struct that has to be present in the code using this macro
struct NonZeroNumber(u32);

#[test]
fn create_non_zero_number_should_compile_for_non_zero_number() {
    non_zero_number!(1);
}
