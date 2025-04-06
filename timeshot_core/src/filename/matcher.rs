
use chrono::NaiveDateTime;

pub fn is_date_in_filename(filename: &str, date: &Option<NaiveDateTime>) -> bool {
    if let Some(date) = date {
        let date_str = date.format("%Y-%m-%d").to_string();
        return filename.contains(&date_str);
    }
    false
}
