use std::io::{self, BufRead};

mod date_parse;

use crate::date_parse::{fix_timestamp_in_line, get_iso_date_regex};

fn main() {
    let date_regex = get_iso_date_regex();

    io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Error reading line"))
        .map(|line| fix_timestamp_in_line(&line, &date_regex))
        .for_each(|line| println!("{}", line));
}
