mod weather;

use crossterm::{
    event::{read, Event, KeyCode},
    style::Stylize,
};
use dotenvy::dotenv;

use std::env;

fn get_api_key() -> String {
    return env::var("API_OPENWEATHER").expect("API key is missing!");
}

fn main() -> Result<(), ureq::Error> {
    dotenv().expect("Unable to find .env file!");
    println!(
        "🦀 Rusty Weather v{} \n\
        Provided API key: {}...",
        env!("CARGO_PKG_VERSION"),
        get_api_key()
            .chars()
            .into_iter()
            .take(9)
            .collect::<String>()
            .green()
    );

    let weather = weather::get_location_data().unwrap();
    println!(
        "\nShowing current weather for {} ({})\n",
        weather.name.cyan(),
        weather.country.dark_cyan()
    );
    let forecast = weather::get_weather(weather.lat, weather.lon).unwrap();
    let conditions = forecast.weather.first().unwrap();
    println!(
        "🌡️  Temperature: {} °C (feels like {} °C) | 💧 Humidity: {}% \n\
        💨 Wind Speed: {} m/s | ☁️  Cloud Percentage: {}% \n\n\
        Weather Condition is {} ({}) \n",
        forecast
            .main
            .get("temp")
            .unwrap()
            .round()
            .to_string()
            .yellow(),
        forecast
            .main
            .get("feels_like")
            .unwrap()
            .round()
            .to_string()
            .yellow(),
        forecast
            .main
            .get("humidity")
            .unwrap()
            .round()
            .to_string()
            .yellow(),
        forecast
            .wind
            .get("speed")
            .unwrap()
            .round()
            .to_string()
            .yellow(),
        forecast
            .clouds
            .get("all")
            .unwrap()
            .round()
            .to_string()
            .yellow(),
        conditions.main.clone().yellow(),
        conditions.description.clone().dark_grey()
    );

    println!("Press Enter to exit...");
    loop {
        let event = read()?;
        if event == Event::Key(KeyCode::Enter.into()) {
            break;
        }
    }

    Ok(())
}
