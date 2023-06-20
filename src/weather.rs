use serde_json::Value;

const WEATHER_URL: &str = "https://api.open-meteo.com/v1/forecast?latitude=LATITUDE&longitude=LONGITUDE&current_weather=true";

pub fn get_weather(latitude: &str, longitude: &str) -> String {
    let weather_request_url = WEATHER_URL
        .replace("LATITUDE", latitude)
        .replace("LONGITUDE", longitude);

    let weather = reqwest::blocking::get(weather_request_url)
        .unwrap()
        .text()
        .unwrap_or("{}".to_string());
    let value: Value = serde_json::from_str(&weather).unwrap();

    format!(
        "{};{};{};{}",
        value["current_weather"]["weathercode"],
        value["current_weather"]["is_day"],
        value["current_weather"]["temperature"],
        value["current_weather"]["windspeed"]
    )
}
