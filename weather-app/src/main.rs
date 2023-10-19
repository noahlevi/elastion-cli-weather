pub mod additional_functions;
pub mod bl;
pub mod cache;
pub mod config;
pub mod structs;
use crate::structs::errors::CustomError;
use clap::Parser;

use crate::bl::{configure_provider, get_weather_forecast};
use crate::structs::{
    cli_args::{SubcommandType, WeatherArgs},
    view::AvailableProviders,
};
use cli_table::{print_stdout, WithTitle};
use config::{OPEN_WEATHER, OPEN_WEATHER_WEBSITE, WEATHER_API, WEATHER_API_WEBSITE};
extern crate dotenv;

// use dotenv::dotenv;
// use std::env;

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    let args: WeatherArgs = WeatherArgs::parse();

    match args.subcommand_ {
        SubcommandType::Configuration(_conf_args) => {
            configure_provider(_conf_args.provider)?;
        }

        SubcommandType::Get(args) => {
            let weather_forecast = get_weather_forecast(args).await?;
            println!("Procesing...");
            print_stdout(vec![weather_forecast].with_title())?;
        }
        SubcommandType::Providers => {
            let providers: Vec<AvailableProviders> = vec![
                AvailableProviders {
                    name: OPEN_WEATHER.to_string(),
                    website: OPEN_WEATHER_WEBSITE.to_string(),
                },
                AvailableProviders {
                    name: WEATHER_API.to_string(),
                    website: WEATHER_API_WEBSITE.to_string(),
                },
            ];
            print_stdout(providers.with_title())?;
        }
    }
    Ok(())
}
