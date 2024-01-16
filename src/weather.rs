use std::collections::HashMap;

use serde::Deserialize;

use crate::get_api_key;

const HTTP_BASE: &str = "https://api.openweathermap.org";

#[derive(Deserialize)]
pub struct NetworkData {
    latitude: f64,
    longitude: f64,
}

#[derive(Deserialize)]
pub struct LocationData {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub country: String,
}

#[derive(Deserialize)]
pub struct WeatherData {
    pub main: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct CurrentForecastData {
    pub weather: Vec<WeatherData>,
    pub main: HashMap<String, f32>,
    pub wind: HashMap<String, f32>,
    pub clouds: HashMap<String, f32>,
}

// almost all openweather apis require key, so we use wrapper for requests
fn get_request(path: String) -> String {
    String::from(HTTP_BASE) + &path + "&appid=" + &get_api_key()
}

// to detect closest location we use service that provides location info for current IP address
// after that we can get required data to give forecast
pub fn get_location_data() -> Result<LocationData, ureq::Error> {
    let network_data: NetworkData = ureq::get("https://ipapi.co/json/").call()?.into_json()?;
    let res: Vec<LocationData> = ureq::get(&get_request(format!(
        "/geo/1.0/reverse?lat={}&lon={}&limit=1",
        network_data.latitude, network_data.longitude
    )))
    .call()?
    .into_json()?;

    Ok(res.into_iter().nth(0).unwrap())
}

pub fn get_weather(lat: f64, lon: f64) -> Result<CurrentForecastData, ureq::Error> {
    let res: CurrentForecastData = ureq::get(&get_request(format!(
        "/data/2.5/weather?lat={}&lon={}&units=metric",
        lat, lon
    )))
    .call()?
    .into_json()?;

    Ok(res)
}
