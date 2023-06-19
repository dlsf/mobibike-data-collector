use chrono::Local;

pub fn get_formatted_time() -> String {
    let local = Local::now();
    local.format("%Y-%m-%d;%H:%M").to_string()
}
