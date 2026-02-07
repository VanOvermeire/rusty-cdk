const VALID_INSTANCE_SIZES: [&str;5] = ["2x", "4x", "8x", "16x", "24x"];

pub fn validate_document_db_instance_class(value: &str) -> Result<(), String> {
    let value_parts: Vec<_> = value.split(".").collect();

    if value_parts.len() != 3 {
        return Err("document db instance class should consist of three parts separated by periods (e.g. `db.t3.medium`)".to_string());
    } else if value_parts[0] != "db" {
        return Err("document db instance class should start with `db.`".to_string());
    }

    let instance_type = value_parts[1];
    
    if instance_type.len() < 2 {
        return Err("document db instance type (part after `db.` and before the size) should be at least two characters long".to_string());
    }
    
    let mut chars_iterator = instance_type.chars();
    let instance_type_first_char = chars_iterator.next().expect("just check that there are two chars");
    let instance_type_second_char = chars_iterator.next().expect("just checked that there are two chars");
    
    if instance_type_first_char != 't' && instance_type_first_char != 'r' {
        return Err(format!("document db instance type should be `t` or `r` (was {})", instance_type));
    }
    
    let instance_type_second_char = instance_type_second_char.to_string().parse::<u8>().map_err(|_| format!("document db instance type should be followed by a number (was {})", instance_type))?;
    
    if instance_type_first_char == 't' && instance_type_second_char < 3 {
        return Err(format!("document db instance type should be of generation t3 or higher (was {})", instance_type_second_char));
    }
    if instance_type_first_char == 'r' && instance_type_second_char < 4 {
        return Err(format!("document db instance type should be of generation r4 or higher (was {})", instance_type_second_char));
    }
    
    let instance_size = value_parts[2];

    if !instance_size.ends_with("medium") && !instance_size.ends_with("large") {
        return Err(format!("document db instance size should be `medium` or `(nx)large` (was {})", instance_size));
    }

    let instance_size_without_suffix = instance_size.replace("medium", "").replace("large", "");
    
    if instance_size.ends_with("medium") && !instance_size_without_suffix.is_empty() {
        return Err(format!("document db instance size should be `medium` or `(nx)large` (was {})", instance_size));
    }

    if !instance_size_without_suffix.is_empty() && !VALID_INSTANCE_SIZES.contains(&instance_size_without_suffix.as_str()) {
        return Err(format!("document db instance size of size large should have one of the following multipliers: {} (was {})", VALID_INSTANCE_SIZES.join(", "),  instance_size));
    }

    Ok(())
}
