use std::f64::consts::PI;

use crate::models::CardinalDirection;

pub fn rad2deg(rad: f64) -> f64 
{
    (rad * 180.0) / PI
}


pub fn deg2cardinal(deg: f64) -> CardinalDirection
{
    let mut shifted = deg + 22.5;
    if shifted > 360.0 {
        shifted = shifted - 360.0; 
    }
    let i = (shifted / 45.0).ceil() as u8;
    CardinalDirection::from(i)
}

