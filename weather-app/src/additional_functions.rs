use crate::config::*;
use crate::structs::errors::CustomError;

use regex::Regex;
use reqwest::Client;

pub fn get_api_client() -> Client {
    let client = Client::new();
    client
}

pub fn to_celsius(temp_far: f32) -> f32 {
    return temp_far - FARINGATE_0;
}

pub fn match_date_string(date: &str) -> Result<(), CustomError> {
    let re = Regex::new(r"^\d{4}\-(0?[1-9]|1[012])\-(0?[1-9]|[12][0-9]|3[01])$").unwrap();
    match re.is_match(date) {
        true => Ok(()),
        false => Err(CustomError::IncorrectDateFormatError),
    }
}
