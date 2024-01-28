use chrono::{DateTime, Local, TimeZone, Utc};
use regex::{Captures, Regex};
use std::io::{self, BufRead};

fn parse_regex_match<T: std::str::FromStr>(caps: &Captures, key: &str) -> Option<T> {
    let found_key = caps.name(key)?;
    let parse_result = found_key.as_str().parse::<T>();

    match parse_result {
        Ok(output) => Some(output),
        Err(_) => panic!("Type mismatch, couldn't convert found key to input type"),
    }
}

fn get_iso_date_regex() -> Regex {
    Regex::new(
        r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})T(?P<hour>\d{2}):(?P<minute>\d{2}):(?P<second>\d{2})(?P<separator>[.:])(?P<post_sep>\d{2,6})"
        ).expect("Regex malformed")
}

fn fix_timestamp_in_line(line: &str, date_regex: &Regex) -> String {
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
        let post_sep = parse_regex_match::<u32>(caps, "post_sep").unwrap();
        let separator = parse_regex_match::<String>(caps, "separator").unwrap();

        let local_date: DateTime<Local> = DateTime::from(utc_date);
        format!(
            "{}{separator}{post_sep}",
            local_date.format("%Y-%m-%dT%H:%M:%S")
        )
    });

    fixed_line.to_string()
}

fn main() {
    let date_regex = get_iso_date_regex();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let text = line.expect("Error reading line");

        let fixed_line = fix_timestamp_in_line(&text, &date_regex);

        println!("{}", fixed_line);
    }
}
