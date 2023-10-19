pub mod additional_functions;
pub mod cache;
pub mod config;
pub mod structs;
pub mod bl;
use clap::Parser;
use crate::structs::errors::CustomError;

use crate::bl::{configure_provider, get_weather_forecast};
use cli_table::{print_stdout, WithTitle};
use crate::structs::cli_args::{SubcommandType, WeatherArgs};
extern crate dotenv;

// use dotenv::dotenv;
// use std::env;

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    let args: WeatherArgs = WeatherArgs::parse();

    match args.subcommand_ {
        SubcommandType::Configuration(_conf_args) => {
            configure_provider(_conf_args.provider)?;
            Ok(())
        }

        SubcommandType::Get(args) => {
            let weather_forecast = get_weather_forecast(args).await?;
            println!("Procesing...");
            print_stdout(vec![weather_forecast].with_title())?;
            Ok(())
        }
    }
}
