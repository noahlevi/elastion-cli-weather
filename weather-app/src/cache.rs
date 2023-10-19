use crate::config::{DEFAULT_PROVIDER, REDIS_PROVIDER_KEY};
use crate::structs::coordinates::Coordinate;
use redis::{Client, Commands, Connection, RedisResult};
use serde_json;
extern crate redis;
use crate::structs::errors::CustomError;

pub fn get_connection() -> RedisResult<Connection> {
    let client: Client = Client::open("redis://127.0.0.1/")?;
    let mut _conn: Connection = client.get_connection()?;

    Ok(_conn)
}

pub fn get_current_provider() -> Result<String, CustomError> {
    let mut conn = get_connection()?;
    let provider: String;
    match conn.get(String::from(REDIS_PROVIDER_KEY)) {
        Ok(p) => provider = p,
        Err(_) => {
            set_current_provider_as_default()?;
            provider = DEFAULT_PROVIDER.to_string();
        }
    }
    Ok(provider)
}

pub fn set_current_provider(provider: String) -> RedisResult<()> {
    let mut conn = get_connection()?;
    Ok(conn.set(String::from(REDIS_PROVIDER_KEY), provider)?)
}

pub fn set_current_provider_as_default() -> Result<(), CustomError> {
    Ok(set_current_provider(String::from(DEFAULT_PROVIDER))?)
}

pub fn get_cached_coordinate(location: &str) -> Result<Option<Coordinate>, CustomError> {
    let mut conn = get_connection()?;
    let json_string: String;
    println!("BEFORE redis get cached coordinate");
    match conn.get(location.to_string()) {
        Ok(json_coord) => {
            json_string = json_coord;
            println!("{}", json_string);
            let coordinate: Coordinate = serde_json::from_str(&json_string).unwrap();
            Ok(Some(coordinate))
        }
        Err(_) => Ok(None),
    }
}

pub fn set_coordinates_to_cache(location: &str, coord: Coordinate) -> Result<(), CustomError> {
    let mut conn = get_connection()?;
    let json_str_coord = serde_json::to_string(&coord)?;
    Ok(conn.set(location.to_string(), json_str_coord)?)
}
