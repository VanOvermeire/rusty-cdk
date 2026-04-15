use syn::{LitInt};

pub fn period_validator(input: &LitInt) -> Result<(), String> {
    let value = input.base10_parse::<u32>().unwrap();

    if value == 10 || value == 20 || value == 30 || value == 60 || (value > 60 && value % 60 == 0) {
        Ok(())
    } else {
        Err(format!("Invalid period. Valid values are 10, 20, 30, 60, and any multiple of 60 (was {}).", value))
    }
}
