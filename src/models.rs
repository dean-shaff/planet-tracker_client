use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AstronObject {
    Mercury,
    Venus,
    Moon,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstronObjectResponse {
    name: AstronObject,
    magnitude: f64,
    size: f64,
    az: f64,
    el: f64,
    ra: f64,
    dec: f64,
    setting_time: NaiveDateTime,
    rising_time: NaiveDateTime,
    when: NaiveDateTime,
}