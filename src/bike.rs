const NEXTBIKE_URL: &str =
    "https://gbfs.nextbike.net/maps/gbfs/v2/nextbike_dx/de/free_bike_status.json";

pub fn get_bike_count(station_format: &str) -> usize {
    let response = reqwest::blocking::get(NEXTBIKE_URL)
        .unwrap()
        .text()
        .unwrap_or(String::new());
    response.matches(station_format).count()
}
