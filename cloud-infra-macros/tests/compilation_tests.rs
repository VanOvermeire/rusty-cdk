#![allow(dead_code)]

use cloud_infra_macros::create_non_zero_number;

// struct that has to be present in the code using this macro
struct NonZeroNumber(u32);

#[test]
fn create_non_zero_number_should_compile_for_non_zero_number() {
    create_non_zero_number!(1);
}
