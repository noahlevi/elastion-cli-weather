use crate::cache::{
    get_cached_coordinate, get_current_provider, set_coordinates_to_cache, set_current_provider,
};
use crate::config::*;
use crate::structs::{
    cli_args::GetForecastArgs,
    coordinates::Coordinate,
    errors::CustomError,
    responses::{
        OpenWeatherForecastCurrentWeatherResponse, OpenWeatherForecastHistoryResponse,
        OpenWeatherGeoResponse, WeatherApiForecastResponse,
    },
    view::UniversalWeather,
};

use crate::additional_functions::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use dotenv::var;
use std::time::SystemTime;

// extern crate requests;
// use requests::ToJson;

pub fn configure_provider(provider: String) -> Result<(), CustomError> {
    let provider_uppercase = provider.to_uppercase();
    if [OPEN_WEATHER, WEATHER_API].contains(&provider_uppercase.as_str()) {
        Ok(set_current_provider(provider_uppercase)?)
    } else {
        Err(CustomError::ProviderDoesntExists)
    }
}

pub async fn get_weather_forecast(args: GetForecastArgs) -> Result<UniversalWeather, CustomError> {
    let current_provider = get_current_provider()?;

    match current_provider.as_str() {
        OPEN_WEATHER => Ok(get_open_weather_forecast(args.date, &args.address).await?),
        WEATHER_API => Ok(get_weather_api_forecast(args.date, &args.address).await?),
        _ => Err(CustomError::ProviderDoesntExists),
    }
}

pub async fn get_open_weather_forecast(
    date: Option<String>,
    location: &str,
) -> Result<UniversalWeather, CustomError> {
    let coordinate: Coordinate;

    match get_cached_coordinate(location)? {
        Some(coord) => {
            coordinate = coord;
        }
        None => {
            coordinate = get_coordinate_from_provider(location).await?;
            set_coordinates_to_cache(location, coordinate.clone())?;
        }
    }

    let api_key = var("OPEN_WEATHER_API_KEY")?;

    let start_hour: u128;

    let url: String;

    let api_client = get_api_client();

    match date {
        Some(d) => {
            println!("{}", d);

            match_date_string(&d)?;

            let date_time = NaiveDateTime::parse_from_str(&d, "%yyyy-%m-%d")?;
            start_hour = date_time.timestamp_millis() as u128;
            // Currently this option is not working, cause of all history data is payable
            println!(" Currently this option is not working");
            url = format!(
                "{}/data/2.5/history/city?lat={}&lon={}&type=hour&start={}&cnt=1&appid={}",
                OPEN_WEATHER_API_BASE.to_string(),
                coordinate.lat,
                coordinate.lon,
                start_hour,
                api_key
            );
            println!("{}", url);
            let res = api_client
                .get(url)
                .send()
                .await?
                .json::<OpenWeatherForecastHistoryResponse>()
                .await?;
            match res.list.get(0) {
                Some(forecast_data) => {
                    let main = &forecast_data.main;
                    let weather = UniversalWeather {
                        feels_like: to_celsius(main.feels_like),
                        temp: to_celsius(main.temp),
                        pressure: main.pressure as f32,
                        humidity: main.humidity as f32,
                        temp_max: to_celsius(main.temp_max),
                        temp_min: to_celsius(main.temp_min),
                    };
                    Ok(weather)
                }
                None => Err(CustomError::WeatherForecastNotFound),
            }
        }
        None => {
            url = format!(
                "{}/data/2.5/weather?lat={}&lon={}&appid={}",
                OPEN_WEATHER_API_BASE.to_string(),
                coordinate.lat,
                coordinate.lon,
                api_key
            );
            let res = api_client
                .get(url)
                .send()
                .await?
                .json::<OpenWeatherForecastCurrentWeatherResponse>()
                .await?;
            let main = res.main;
            let weather = UniversalWeather {
                feels_like: to_celsius(main.feels_like),
                temp: to_celsius(main.temp),
                pressure: main.pressure as f32,
                humidity: main.humidity as f32,
                temp_max: to_celsius(main.temp_max),
                temp_min: to_celsius(main.temp_min),
            };
            Ok(weather)
        }
    }
}

pub async fn get_weather_api_forecast(
    date: Option<String>,
    location: &str,
) -> Result<UniversalWeather, CustomError> {
    let api_client = get_api_client();

    let api_key = var("WEATHERAPI_API_KEY")?;

    let date_q: String;

    match date {
        Some(ds) => {
            match_date_string(&ds)?;
            date_q = ds
        }
        None => {
            let system_time = SystemTime::now();
            let datetime: DateTime<Utc> = system_time.into();
            date_q = datetime.format("%Y-%m-%d").to_string();
        }
    };

    let url = format!(
        "{}/history.json?key={}&q={}&dt={}",
        WEATHER_API_BASE.to_string(),
        api_key,
        location.to_string(),
        date_q
    );

    println!("{}", url);

    let res = api_client
        .get(url)
        .send()
        .await?
        .json::<WeatherApiForecastResponse>()
        .await?;
    let weather: UniversalWeather;
    match res.forecast.forecastday.get(0) {
        Some(day_data) => {
            weather = UniversalWeather {
                feels_like: day_data.day.avgtemp_c,
                temp: day_data.day.avgtemp_c,
                pressure: day_data.day.totalprecip_mm,
                humidity: day_data.day.avghumidity,
                temp_max: day_data.day.maxtemp_c,
                temp_min: day_data.day.maxtemp_c,
            };
            Ok(weather)
        }
        None => Err(CustomError::WeatherForecastNotFound),
    }
}

pub async fn get_coordinate_from_provider(location: &str) -> Result<Coordinate, CustomError> {
    let api_client = get_api_client();

    let api_key = var("OPEN_WEATHER_API_KEY")?;

    let res = api_client
        .get(format!(
            "{}/geo/1.0/direct?q={}&limit=1&appid={}&=",
            OPEN_WEATHER_API_BASE.to_string(),
            location.to_string(),
            api_key
        ))
        .send()
        .await?
        .json::<Vec<OpenWeatherGeoResponse>>()
        .await?;
    println!("AFTER GEO req");

    match res.get(0) {
        Some(geo_res) => {
            let coordinate = Coordinate {
                lat: geo_res.lat,
                lon: geo_res.lon,
            };
            Ok(coordinate)
        }
        None => {
            return Err(CustomError::CoordinatesDoesnExistsError);
        }
    }
}
