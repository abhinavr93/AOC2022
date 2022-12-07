use regex::Regex;
use std::io::{Error, ErrorKind};

pub fn parse_day_num(day_str: &str) -> Result<u32, Error> {
    let day_regex = Regex::new(r"^day(\d{2})$").expect("Invalid regex");
    let num;
    if let Some(caps) = day_regex.captures(&day_str) {
        num = caps
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .expect("Invalid day input");
        if num > 25 {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid day number"));
        }
    } else {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid day name"));
    }
    Ok(num)
}
