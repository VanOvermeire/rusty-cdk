use quote::__private::Span;
use syn::Error;

fn error_with_message(span: Span) -> impl Fn(String) -> Result<(), Error> {
    move |message| Err(Error::new(span, message))
}

fn valid_number_between(to_parse: &str, min: u16, max: u16) -> bool {
    if to_parse == "*" {
        return true;
    }

    let parsed: Result<u16, _> = to_parse.parse();
    match parsed {
        Ok(v) => {
            if v < min || v > max {
                false
            } else {
                true
            }
        }
        Err(_) => false,
    }
}

const BASIC_CRON_WILDCARDS: [&str; 4] = [",", "-", "*", "/"];
const MONTH_CRON_WILDCARDS: [&str; 7] = [",", "-", "*", "/", "?", "L", "W"];
const WEEK_CRON_WILDCARDS: [&str; 6] = [",", "-", "*", "?", "L", "#"];
const DAY_NAMES: [&str; 7] = ["MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN"];
const MONTH_NAMES: [&str; 12] = ["JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC"];

// covers most, but not all cases
pub(crate) fn validate_cron(value: &str, span: Span) -> Result<(), Error> {
    let with_message = error_with_message(span);
    let value_parts: Vec<_> = value.split(" ").collect();

    if value_parts.len() < 5 {
        return with_message(format!(
            "cron expression should consist of five or six fields (was {})",
            value_parts.len()
        ));
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
            return with_message(format!(
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
            return with_message(format!(
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
            return with_message(format!(
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
        let invalid_names = month_parts.iter().any(|v| !MONTH_NAMES.contains(&v));
        let invalid_numbers = month_parts.iter().any(|v| !valid_number_between(v, 1, 12));

        if invalid_names && invalid_numbers {
            return with_message(format!(
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
        let invalid_names = day_of_week_parts.iter().any(|v| !DAY_NAMES.contains(&v));
        let invalid_numbers = day_of_week_parts.iter().any(|m| !valid_number_between(m, 1, 7));
        if invalid_names && invalid_numbers {
            return with_message(format!(
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
                return with_message(format!(
                    "year should be numbers between 1970 and 2199 and/or these wildcards: {} (was {})",
                    BASIC_CRON_WILDCARDS.join(" "),
                    year
                ));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::cron_validation::validate_cron;
    use quote::__private::Span;

    #[test]
    fn validate_cron_every_minutes() {
        let validated = validate_cron("* * * * *", Span::call_site());

        assert!(validated.is_ok());
    }

    #[test]
    fn validate_cron_every_two_minutes() {
        let validated = validate_cron("*/2 * * * *", Span::call_site());

        assert!(validated.is_ok());
    }

    #[test]
    fn validate_cron_every_tenth_minute() {
        let validated = validate_cron("*/10 * * * *", Span::call_site());

        assert!(validated.is_ok());
    }

    #[test]
    fn validate_cron_every_three_hours() {
        let validated = validate_cron("0 */3 * * *", Span::call_site());

        assert!(validated.is_ok());
    }

    #[test]
    fn validate_cron_every_day_at_2() {
        let validated = validate_cron("0 2 * * *", Span::call_site());

        assert!(validated.is_ok());
    }

    #[test]
    fn validate_cron_every_thursday() {
        let validated = validate_cron("0 0 * * THU", Span::call_site());

        assert!(validated.is_ok());
    }

    #[test]
    fn validate_very_specific_cron() {
        let validated = validate_cron("15 3 ? JAN-DEC MON/THU 2050", Span::call_site());

        assert!(validated.is_ok());
    }

    #[test]
    fn validate_invalid_minutes() {
        let validated = validate_cron("- 0 * * *", Span::call_site());

        assert!(validated.is_err());
    }

    #[test]
    fn validate_invalid_minutes_too_high() {
        let validated = validate_cron("66 0 * * *", Span::call_site());

        assert!(validated.is_err());
    }

    #[test]
    fn validate_invalid_hours() {
        let validated = validate_cron("0 F * * *", Span::call_site());

        assert!(validated.is_err());
    }

    #[test]
    fn validate_invalid_month() {
        let validated = validate_cron("0 * * JUN/DOC *", Span::call_site());

        assert!(validated.is_err());
    }

    #[test]
    fn validate_invalid_day_of_week_too_low() {
        let validated = validate_cron("0 * * * 0", Span::call_site());

        assert!(validated.is_err());
    }
}
