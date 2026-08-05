use chrono::Local;

pub fn create_file_name() -> String {
    let now = Local::now();
    format!("{}.jpg", now.format("%Y%m%d_%H%M%S"))
}
