fn valid_number_between(to_parse: &str, min: u16, max: u16) -> bool {
    if to_parse == "*" {
        return true;
    }

    let parsed: Result<u16, _> = to_parse.parse();
    match parsed {
        Ok(v) => v < min || v > max,
        Err(_) => false,
    }
}

const BASIC_CRON_WILDCARDS: [&str; 4] = [",", "-", "*", "/"];
const MONTH_CRON_WILDCARDS: [&str; 7] = [",", "-", "*", "/", "?", "L", "W"];
const WEEK_CRON_WILDCARDS: [&str; 6] = [",", "-", "*", "?", "L", "#"];
const DAY_NAMES: [&str; 7] = ["MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN"];
const MONTH_NAMES: [&str; 12] = ["JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC"];

// covers most, but not all cases
pub(crate) fn validate_cron(value: &str) -> Result<(), String> {
    let value_parts: Vec<_> = value.split(" ").collect();

    if value_parts.len() < 5 {
        return Err(format!("cron expression should consist of five or six fields (was {})", value_parts.len()));
    }

    let minutes = value_parts[0];

    if minutes != "*" {
        let minutes_parts = if minutes.contains(",") {
            minutes.split(",").collect()
        } else if minutes.contains("-") {
            minutes.split("-").collect()
        } else if minutes.contains("/") {
            minutes.split("/").collect()
        } else {
            vec![minutes]
        };
        let invalid = minutes_parts.iter().any(|v| !valid_number_between(v, 0, 59));

        if invalid {
            return Err(format!(
                "minutes should be numbers between 0 and 59 and/or these wildcards: {} (was {})",
                BASIC_CRON_WILDCARDS.join(" "),
                minutes
            ));
        }
    }
    let hours = value_parts[1];

    if hours != "*" {
        let hours_parts = if hours.contains(",") {
            hours.split(",").collect()
        } else if hours.contains("-") {
            hours.split("-").collect()
        } else if hours.contains("/") {
            hours.split("/").collect()
        } else {
            vec![hours]
        };
        let invalid = hours_parts.iter().any(|v| !valid_number_between(v, 0, 23));

        if invalid {
            return Err(format!(
                "hours should be numbers between 0 and 23 and/or these wildcards: {} (was {})",
                BASIC_CRON_WILDCARDS.join(" "),
                hours
            ));
        }
    }

    let day_of_month = value_parts[2];

    if day_of_month != "*" && day_of_month != "L" && day_of_month != "?" {
        let day_of_month_parts = if day_of_month.contains(",") {
            day_of_month.split(",").collect()
        } else if day_of_month.contains("-") {
            day_of_month.split("-").collect()
        } else if day_of_month.contains("/") {
            day_of_month.split("/").collect()
        } else if day_of_month.contains("W") {
            day_of_month.split("W").collect()
        } else {
            vec![day_of_month]
        };
        let invalid = day_of_month_parts.iter().any(|v| !valid_number_between(v, 1, 31));

        if invalid {
            return Err(format!(
                "day of month should be numbers between 1 and 31 and/or these wildcards: {} (was {})",
                MONTH_CRON_WILDCARDS.join(" "),
                day_of_month
            ));
        }
    }

    let month = value_parts[3];

    if month != "*" {
        let month_parts = if month.contains(",") {
            month.split(",").collect()
        } else if month.contains("-") {
            month.split("-").collect()
        } else if month.contains("/") {
            month.split("/").collect()
        } else {
            vec![month]
        };
        let invalid_names = month_parts.iter().any(|v| !MONTH_NAMES.contains(v));
        let invalid_numbers = month_parts.iter().any(|v| !valid_number_between(v, 1, 12));

        if invalid_names && invalid_numbers {
            return Err(format!(
                "month should be numbers between 1 and 12, or these names {} and/or these wildcards: {} (was {})",
                MONTH_NAMES.join(","),
                BASIC_CRON_WILDCARDS.join(" "),
                month
            ));
        }
    }

    let day_of_week = value_parts[4];

    if day_of_week != "*" && day_of_week != "L" && day_of_week != "?" {
        let day_of_week_parts = if day_of_week.contains(",") {
            day_of_week.split(",").collect()
        } else if day_of_week.contains("-") {
            day_of_week.split("-").collect()
        } else if day_of_week.contains("/") {
            day_of_week.split("/").collect()
        } else if day_of_week.contains("#") {
            day_of_week.split("#").collect()
        } else {
            vec![day_of_week]
        };
        let invalid_names = day_of_week_parts.iter().any(|v| !DAY_NAMES.contains(v));
        let invalid_numbers = day_of_week_parts.iter().any(|m| !valid_number_between(m, 1, 7));
        if invalid_names && invalid_numbers {
            return Err(format!(
                "day of week should be numbers between 1 and 7, or these names {} and/or these wildcards: {} (was {})",
                DAY_NAMES.join(","),
                WEEK_CRON_WILDCARDS.join(" "),
                day_of_week
            ));
        }
    }

    if value_parts.len() == 6 {
        let year = value_parts[5];

        if year != "*" {
            let year_parts = if year.contains(",") {
                year.split(",").collect()
            } else if year.contains("-") {
                year.split("-").collect()
            } else if year.contains("/") {
                year.split("/").collect()
            } else {
                vec![year]
            };
            let invalid = year_parts.iter().any(|v| !valid_number_between(v, 1970, 2199));

            if invalid {
                return Err(format!(
                    "year should be numbers between 1970 and 2199 and/or these wildcards: {} (was {})",
                    BASIC_CRON_WILDCARDS.join(" "),
                    year
                ));
            }
        }
    }

    Ok(())
}

pub(crate) fn validate_at(value: &str) -> Result<(), String> {
    let split: Vec<_> = value.split('T').collect();

    if split.len() != 2 {
        return Err("`at` should be a date followed by a time, separated by `T` (yyyy-mm-ddThh:mm:ss)".to_string());
    }

    let date = split[0];
    let time = split[1];

    let date_parts: Vec<_> = date.split('-').collect();
    
    if date_parts.len() != 3 {
        return Err("`at` date should be year, month, day, separated by `-` (yyyy-mm-ddThh:mm:ss)".to_string());
    }

    let years = date_parts[0];
    let months = date_parts[1];
    let days = date_parts[2];

    if !valid_number_between(years, 1970, 2199) {
        return Err(format!("year should be between 1970 and 2199 (was {})", years));
    }
    if !valid_number_between(months, 1, 12) {
        return Err(format!("months should be between 1 and 12 (was {})", months));
    }
    if !valid_number_between(days, 1, 31) {
        return Err(format!("days should be between 1 and 31 (was {})", days));
    }
    
    let time_parts: Vec<_> = time.split(':').collect();

    if time_parts.len() != 3 {
        return Err("`at` time should be year, month, day, separated by `:` (yyyy-mm-ddThh:mm:ss)".to_string());
    }

    let hours = time_parts[0];
    let minutes = time_parts[1];
    let seconds = time_parts[2];

    if !valid_number_between(hours, 0, 24) {
        return Err(format!("hours should be between 0 and 24 (was {})", hours));
    }
    if !valid_number_between(minutes, 0, 60) {
        return Err(format!("minutes should be between 0 and 60 (was {})", minutes));
    }
    if !valid_number_between(seconds, 0, 60) {
        return Err(format!("seconds should be between 0 and 60 (was {})", seconds));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::schedule_validation::validate_cron;

    #[test]
    fn validate_cron_every_minutes() {
        let validated = validate_cron("* * * * *");

        assert!(validated.is_ok());
    }

    #[test]
    fn validate_cron_every_two_minutes() {
        let validated = validate_cron("*/2 * * * *");

        assert!(validated.is_ok());
    }

    #[test]
    fn validate_cron_every_tenth_minute() {
        let validated = validate_cron("*/10 * * * *");

        assert!(validated.is_ok());
    }

    #[test]
    fn validate_cron_every_three_hours() {
        let validated = validate_cron("0 */3 * * *");

        assert!(validated.is_ok());
    }

    #[test]
    fn validate_cron_every_day_at_2() {
        let validated = validate_cron("0 2 * * *");

        assert!(validated.is_ok());
    }

    #[test]
    fn validate_cron_every_thursday() {
        let validated = validate_cron("0 0 * * THU");

        assert!(validated.is_ok());
    }

    #[test]
    fn validate_very_specific_cron() {
        let validated = validate_cron("15 3 ? JAN-DEC MON/THU 2050");

        assert!(validated.is_ok());
    }

    #[test]
    fn validate_invalid_minutes() {
        let validated = validate_cron("- 0 * * *");

        assert!(validated.is_err());
    }

    #[test]
    fn validate_invalid_minutes_too_high() {
        let validated = validate_cron("66 0 * * *");

        assert!(validated.is_err());
    }

    #[test]
    fn validate_invalid_hours() {
        let validated = validate_cron("0 F * * *");

        assert!(validated.is_err());
    }

    #[test]
    fn validate_invalid_month() {
        let validated = validate_cron("0 * * JUN/DOC *");

        assert!(validated.is_err());
    }

    #[test]
    fn validate_invalid_day_of_week_too_low() {
        let validated = validate_cron("0 * * * 0");

        assert!(validated.is_err());
    }
}
