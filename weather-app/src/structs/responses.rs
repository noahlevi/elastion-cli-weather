use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct OpenWeatherGeoResponse {
    name: String,
    local_names: HashMap<String, String>,
    pub lat: f32,
    pub lon: f32,
    country: String
}

#[derive(Deserialize)]
pub struct OpenWeatherForecastHistoryResponse {
    massage: String,
    cod: String,
    city_id: u64,
    calctime: f32,
    cnt: u64,
    pub list: Vec<OpenWeatherForecastData>
}

#[derive(Deserialize)]
pub struct OpenWeatherForecastCurrentWeatherResponse {
    pub main: OpenWeatherForecastMain,
}

#[derive(Deserialize)]
pub struct OpenWeatherForecastData {
    dt: u64,
    pub main: OpenWeatherForecastMain,
    wind: OpenWeatherForecastWind,
    clouds: OpenWeatherForecastClouds,
    weather: OpenWeatherForecastWeather,
    rain: OpenWeatherForecastRain,
}

#[derive(Deserialize)]
pub struct OpenWeatherForecastMain {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: u64,    
    pub humidity: u64,
}

#[derive(Deserialize)]
pub struct OpenWeatherForecastWind {
    pub speed: f32,
    pub deg: u64,
}

#[derive(Deserialize)]
pub struct OpenWeatherForecastClouds {
    pub all: i16
}

#[derive(Deserialize)]
pub struct OpenWeatherForecastWeather {
    pub id: u64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize)]
pub struct OpenWeatherForecastSys{
    #[serde(rename = "type")]
    typ: u64,
    id: u64,
    country: String,
    sunrise: u64,
    sunset: u64
}

#[derive(Deserialize)]
pub struct OpenWeatherForecastRain (f32);

#[derive(Deserialize)]
pub struct WeatherApiForecastResponse {
    pub forecast: WeatherApiForecast,
}

#[derive(Deserialize)]
pub struct WeatherApiForecast {
    pub forecastday: Vec<WeatherApiForecastDay>,
}

#[derive(Deserialize)]
pub struct WeatherApiForecastDay {
    pub day: WeatherApiForecastDayDay,
}

#[derive(Deserialize)]
pub struct WeatherApiForecastDayDay {
    pub maxtemp_c: f32,
    pub mintemp_c: f32,
    pub avgtemp_c: f32,
    pub avghumidity: f32,
    pub totalprecip_mm: f32
}