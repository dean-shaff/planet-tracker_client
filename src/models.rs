use std::fmt;

use chrono::NaiveDateTime;
use enum_iterator::Sequence;
use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQueryParams {
    pub q: String,
    pub max_results: i64,
    pub fuzzy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchItem {
    pub country: String,
    pub sub_division: String,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pub lat: f64,
    pub lon: f64,
    pub elevation: f64,
}

impl From<SearchItem> for Position {
    fn from(value: SearchItem) -> Self {
        Self {
            lat: value.lat,
            lon: value.lon,
            elevation: 0.0,
        }
    }
}

#[wasm_bindgen]
extern "C" {
    type GeolocationCoordinates;

    #[wasm_bindgen(method, getter)]
    fn latitude(this: &GeolocationCoordinates) -> f64;

    #[wasm_bindgen(method, getter)]
    fn longitude(this: &GeolocationCoordinates) -> f64;

    type GeolocationPosition;

    #[wasm_bindgen(method, getter)]
    fn coords(this: &GeolocationPosition) -> GeolocationCoordinates;
}

fn geo_callback(position: JsValue) -> GeolocationCoordinates {
    let pos = JsCast::unchecked_into::<GeolocationPosition>(position);
    let coords = pos.coords();
    coords
}

impl From<&GeolocationCoordinates> for Position {
    fn from(value: &GeolocationCoordinates) -> Position {
        Position {
            lon: value.longitude(),
            lat: value.latitude(),
            elevation: 0.0,
        }
    }
}

impl Position {
    pub fn from_browser() -> Result<ReadSignal<Option<Self>>, AppError> {
        let window =
            web_sys::window().ok_or(AppError::DomError("Couldn't get the window!".to_string()))?;
        let navigator = window.navigator();
        let geolocation = navigator
            .geolocation()
            .map_err(|_err| AppError::DomError("Couldn't get geolocation".to_string()))?;

        let (coords, set_coords) = create_signal::<Option<Self>>(None);
        create_effect(move |_| {
            let geo_callback_function = Closure::wrap(Box::new(move |pos| {
                let geo_coords = geo_callback(pos);
                set_coords.set(Some(Position::from(&geo_coords)));
            }) as Box<dyn Fn(JsValue)>);
            geolocation
                .get_current_position(&geo_callback_function.as_ref().unchecked_ref())
                .expect("Unable to get position");
            // geolocation
            //     .watch_position(&geo_callback_function.as_ref().unchecked_ref())
            //     .expect("Unable to get position");
            geo_callback_function.forget();
        });
        Ok(coords)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub items: Vec<SearchItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstronObjectQueryParams {
    pub name: AstronObject,
    pub lon: f64,
    pub lat: f64,
    pub elevation: f64,
    pub when: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Sequence, PartialEq, Eq, Hash)]
pub enum AstronObject {
    Sun,
    Mercury,
    Venus,
    Mars,
    Moon,
    Jupiter,
    Saturn,
    // Uranus,
    // Neptune
}

impl AstronObject {
    /// get rgb color associated with this planet or planet-like object
    pub fn get_color<'a>(&self) -> &'a str {
        match self {
            Self::Sun => "rgb(255,204,0)",
            Self::Mercury => "rgb(215,179,119)",
            Self::Venus => "rgb(171,99,19)",
            Self::Mars => "rgb(114,47,18)",
            Self::Moon => "rgba(128,128,128)",
            Self::Jupiter => "rgb(150,81,46)",
            Self::Saturn => "rgb(215,179,119)",
            // Self::Uranus => "rgb(195,233,236)",
            // Self::Neptune => "rgb(71,114,255)",
        }
    }
}

impl fmt::Display for AstronObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sun => write!(f, "Sun"),
            Self::Mercury => write!(f, "Mercury"),
            Self::Venus => write!(f, "Venus"),
            Self::Moon => write!(f, "Moon"),
            Self::Mars => write!(f, "Mars"),
            Self::Jupiter => write!(f, "Jupiter"),
            Self::Saturn => write!(f, "Saturn"),
            // Self::Uranus => write!(f, "Uranus"),
            // Self::Neptune => write!(f, "Neptune"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstronObjectResponse {
    pub name: AstronObject,
    pub magnitude: f64,
    pub size: f64,
    pub az: f64,
    pub el: f64,
    pub ra: f64,
    pub dec: f64,
    pub setting_time: NaiveDateTime,
    pub rising_time: NaiveDateTime,
    pub when: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct SelectedAstronObjectResponse {
    pub selected: bool,
    pub obj: AstronObjectResponse,
}

#[derive(Debug)]
pub enum CardinalDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl fmt::Display for CardinalDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            match self {
                Self::North => write!(f, "N"),
                Self::NorthEast => write!(f, "NE"),
                Self::East => write!(f, "E"),
                Self::SouthEast => write!(f, "SE"),
                Self::South => write!(f, "S"),
                Self::SouthWest => write!(f, "SW"),
                Self::West => write!(f, "W"),
                Self::NorthWest => write!(f, "NW"),
            }
        } else {
            match self {
                Self::North => write!(f, "North"),
                Self::NorthEast => write!(f, "Northeast"),
                Self::East => write!(f, "East"),
                Self::SouthEast => write!(f, "Southeast"),
                Self::South => write!(f, "South"),
                Self::SouthWest => write!(f, "Southwest"),
                Self::West => write!(f, "West"),
                Self::NorthWest => write!(f, "Northwest"),
            }
        }
    }
}

impl From<u8> for CardinalDirection {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::North,
            2 => Self::NorthEast,
            3 => Self::East,
            4 => Self::SouthEast,
            5 => Self::South,
            6 => Self::SouthWest,
            7 => Self::West,
            8 => Self::NorthWest,
            _ => panic!("Can't match value greater than 7!"),
        }
    }
}
