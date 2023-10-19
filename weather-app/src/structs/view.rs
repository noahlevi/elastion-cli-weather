use cli_table::{format::Justify, Table};

#[derive(Table)]
pub struct UniversalWeather {
    #[table(title = "Temperature, ℃", justify = "Justify::Center")]
    pub temp: f32,
    #[table(title = "Feels like, ℃", justify = "Justify::Center")]
    pub feels_like: f32,
    #[table(title = "Pressure", justify = "Justify::Center")]
    pub pressure: f32,
    #[table(title = "Humidity", justify = "Justify::Center")]
    pub humidity: f32,
    #[table(title = "min Temperature, ℃", justify = "Justify::Center")]
    pub temp_min: f32,
    #[table(title = "max Temperature, ℃", justify = "Justify::Center")]
    pub temp_max: f32,    
}
