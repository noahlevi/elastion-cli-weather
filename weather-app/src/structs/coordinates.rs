use redis_derive::{FromRedisValue, ToRedisArgs};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRedisValue, PartialEq, Serialize, Deserialize, ToRedisArgs, Clone)]
pub struct Coordinate {
    pub lat: f32,
    pub lon: f32,
}