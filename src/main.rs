use std::io::{self, BufRead};
use regex::{Captures, Regex};
use chrono::{DateTime, TimeZone, Utc, Local};


fn parse_regex_match<T: std::str::FromStr>(caps: &Captures, key: &str) -> Option<T> {
    let found_key = caps.name(key)?;
    let parse_result = found_key.as_str()
                                .parse::<T>();

    match parse_result {
        Ok(output) => return Some(output),
        Err(_) => panic!("Type mismatch, couldn't convert found key to input type")
    }
}

fn main() {
    let date_regex = Regex::new(
        r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})T(?P<hour>\d{2}):(?P<minute>\d{2}):(?P<second>\d{2})(?P<micro_separator>[.:])(?P<micro>\d{2,6})"
        ).expect("Regex malformed");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let text = line.expect("Error reading line");

        let fixed_line = date_regex.replace(&text, |caps: &Captures| {
            // These "unwraps" are ok because we wouldn't have entered this closure without a match
            // If we have a match, we have the groups
            // if unwrap panics, you typed a key name wrong
            let utc_date = Utc.with_ymd_and_hms(parse_regex_match::<i32>(&caps, "year").unwrap(),  
                                                parse_regex_match::<u32>(&caps, "month").unwrap(), 
                                                parse_regex_match::<u32>(&caps, "day").unwrap(), 
                                                parse_regex_match::<u32>(&caps, "hour").unwrap(), 
                                                parse_regex_match::<u32>(&caps, "minute").unwrap(), 
                                                parse_regex_match::<u32>(&caps, "second").unwrap(), 
                                               ).unwrap();

            // Dealing with micros a little differently, as we assume they aren't touched by timezone conversion
            let micros = parse_regex_match::<u32>(&caps, "micro").unwrap();
            let micros_separator = parse_regex_match::<String>(&caps, "micro_separator").unwrap();

            let local_date : DateTime<Local> = DateTime::from(utc_date);
            format!("{}{micros_separator}{micros}", local_date.format("%Y-%m-%dT%H:%M:%S"))
        });

        println!("{}", fixed_line.as_ref());
    }
}
