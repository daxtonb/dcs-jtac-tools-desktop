use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Level1UnitType {
    AIR = 'A' as isize,
    GROUND = 'G' as isize,
    SEA = 'S' as isize
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum AirLevel2UnitType {
    FIXED_WING = 'F' as isize,
    ROTARY_WING = 'H' as isize
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Coalition {
    NEUTRAL = 0,
    REDFOR = 1,
    BLUFOR = 2
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UnitType {
    pub level_1: char,
    pub level_2: char,
    pub level_3: char,
    pub level_4: char,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Position3D {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DcsUnit {
    pub unit_name: String,
    pub group_name: String,
    pub coalition: Coalition,
    pub position: Position3D,
    pub unit_type: UnitType,
    pub date: String,
    pub mission_start_time: i32,
    pub mission_time_elapsed: i32
} 

