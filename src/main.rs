use std::fs;
use std::time::Duration;

mod bike;
mod file_writer;
mod time;
mod weather;

fn main() {
    let file_content = fs::read_to_string("config.toml").expect("Please create a config.toml file");

    let station_id =
        read_value(&file_content, "STATION_ID").expect("STATION_ID not found in config");
    let station_format = "\"station_id\":\"".to_string() + &station_id;

    let refresh_rate_str =
        read_value(&file_content, "REFRESH_RATE").expect("REFRESH_RATE not found in config");
    let refresh_rate = refresh_rate_str
        .parse::<u64>()
        .expect("Refresh rate is not a number");

    let latitude = read_value(&file_content, "LATITUDE").expect("LATITUDE not found in config");

    let longitude = read_value(&file_content, "LONGITUDE").expect("LONGITUDE not found in config");

    loop {
        println!("Querying {station_id}...");
        let bike_count = bike::get_bike_count(&station_format);
        let weather = weather::get_weather(&latitude, &longitude);

        println!(
            "{}: Found {} mobibike(s) at station {}\nWeather: {}",
            time::get_formatted_time().replace(';', ", "),
            bike_count,
            station_id,
            weather
        );

        file_writer::append(
            "data.csv",
            &format!(
                "{};{};{}\n",
                time::get_formatted_time(),
                bike_count,
                weather
            ),
        );

        println!("Wrote data to file, sleeping...");
        std::thread::sleep(Duration::from_secs(60 * refresh_rate));
    }
}

fn read_value(config_content: &str, key: &str) -> Option<String> {
    config_content
        .lines()
        .find(|line| line.starts_with(key))
        .map(|x| x.split('=').last().unwrap().to_string())
}
