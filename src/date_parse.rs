use chrono::{DateTime, Local, TimeZone, Utc};
use regex::{Captures, Regex};

pub fn parse_regex_match<T: std::str::FromStr>(caps: &Captures, key: &str) -> Option<T> {
    let found_key = caps.name(key)?;
    let parse_result = found_key.as_str().parse::<T>();

    match parse_result {
        Ok(output) => Some(output),
        Err(_) => panic!("Type mismatch, couldn't convert found key to input type"),
    }
}

pub fn get_iso_date_regex() -> Regex {
    Regex::new(
        r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})T(?P<hour>\d{2}):(?P<minute>\d{2}):(?P<second>\d{2})(?P<everything_after>Z|(\.\d+))?"
    ).expect("Regex malformed")
}

pub fn fix_timestamp_in_line(line: &str, date_regex: &Regex) -> String {
    let fixed_line = date_regex.replace(line, |caps: &Captures| {
        // These "unwraps" are ok because we wouldn't have entered this closure without a match
        // If we have a match, we have the groups
        // if unwrap panics, you typed a key name wrong
        let utc_date = Utc
            .with_ymd_and_hms(
                parse_regex_match::<i32>(caps, "year").unwrap(),
                parse_regex_match::<u32>(caps, "month").unwrap(),
                parse_regex_match::<u32>(caps, "day").unwrap(),
                parse_regex_match::<u32>(caps, "hour").unwrap(),
                parse_regex_match::<u32>(caps, "minute").unwrap(),
                parse_regex_match::<u32>(caps, "second").unwrap(),
            )
            .unwrap();

        // Dealing with post separator a little differently, as we assume they aren't touched by timezone conversion
        let local_date: DateTime<Local> = DateTime::from(utc_date);
        let formatted_date = local_date.format("%Y-%m-%dT%H:%M:%S").to_string();
        if let Some(all_fractionals) = parse_regex_match::<String>(caps, "everything_after") {
            format!("{}{}", formatted_date, all_fractionals)
        } else {
            formatted_date
        }
    });

    fixed_line.to_string()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // TODO don't hardcode the timezones

    #[test]
    fn test_fix_timestamp() {
        let date_regex = get_iso_date_regex();
        let line = "2024-01-27T04:15:46.280000Z";
        let fixed_line = fix_timestamp_in_line(line, &date_regex);
        assert_eq!(fixed_line, "2024-01-26T20:15:46.280000Z");
    }

    #[test]
    fn test_fix_timestamp_millis() {
        let date_regex = get_iso_date_regex();
        let line = "2024-01-29T23:21:38Z";
        let fixed_line = fix_timestamp_in_line(line, &date_regex);
        assert_eq!(fixed_line, "2024-01-29T15:21:38Z");
    }
}
