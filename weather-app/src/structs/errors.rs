use chrono;
use core::fmt;
use dotenv;
use redis::RedisError;
use reqwest;
use serde_json;
use std::io;

// use snafu::prelude::*;

#[derive(Debug)]
pub enum CustomError {
    ReqwestError,
    RedisError,
    DotEnvVarError,
    DotEnvIoError,
    DotEnvLineParseError,
    DotEnvNonexhaustiveError,
    DatetimeParseError,
    CoordinatesDoesnExistsError,
    WeatherForecastNotFound,
    ProviderDoesntExists,
    TableReprError,
    SerdeJsonError,
    IncorrectDateFormatError
}

// impl fmt::Display for CustomError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             &CustomError::ProviderDoesntExists => {
//                 Ok(println!("Not a weather provider. Please choose correct"))
//             },
//             _ => Ok(println!("Technical error"))
//         }
//     }
// }

impl From<RedisError> for CustomError {
    fn from(_re: RedisError) -> Self {
        Self::RedisError
    }
}

impl From<reqwest::Error> for CustomError {
    fn from(_re: reqwest::Error) -> Self {
        println!("{}", _re);
        Self::ReqwestError
    }
}

impl From<chrono::ParseError> for CustomError {
    fn from(_pe: chrono::ParseError) -> Self {
        Self::DatetimeParseError
    }
}

impl From<dotenv::Error> for CustomError {
    fn from(de: dotenv::Error) -> Self {
        match de {
            dotenv::Error::EnvVar(_) => Self::DotEnvVarError,
            dotenv::Error::Io(_) => Self::DotEnvIoError,
            dotenv::Error::LineParse(_, _) => Self::DotEnvLineParseError,
            dotenv::Error::__Nonexhaustive => Self::DotEnvNonexhaustiveError,
        }
    }
}

impl From<io::Error> for CustomError {
    fn from(_ioe: io::Error) -> Self {
        Self::TableReprError
    }
}

impl From<serde_json::Error> for CustomError {
    fn from(_re: serde_json::Error) -> Self {
        Self::SerdeJsonError
    }
}
