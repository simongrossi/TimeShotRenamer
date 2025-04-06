
use chrono::NaiveDateTime;
use regex::Regex;

pub enum DateDetection {
    NotFound,
    FoundValidFormat(NaiveDateTime),
    FoundButBadFormat(String),
}

pub fn detect_date_pattern_in_filename(filename: &str) -> DateDetection {
    let re_patterns = vec![
        (r"(\d{4})[-_]?(\d{2})[-_]?(\d{2})[-_]?(\d{2})(\d{2})(\d{2})", "%Y%m%d%H%M%S"),
        (r"(\d{4})[-_.]?(\d{2})[-_.]?(\d{2})", "%Y%m%d"),
        (r"(\d{2})[-_.]?(\d{2})[-_.]?(\d{4})", "%d%m%Y"),
        (r"(\d{4})[^\d]?(\d{2})[^\d]?(\d{2})", "%Y%m%d"),
        (r"(\d{4})[-_](\d{2})[-_](\d{2})[_-](\d{2})(\d{2})(\d{2})", "%Y-%m-%d_%H%M%S"),
    ];

    for (pattern, format) in re_patterns {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(caps) = re.captures(filename) {
                let date_str = caps.iter()
                    .skip(1)
                    .filter_map(|c| c.map(|m| m.as_str()))
                    .collect::<Vec<_>>()
                    .join("");

                if let Ok(parsed_date) = NaiveDateTime::parse_from_str(&date_str, format) {
                    if format == "%Y-%m-%d_%H%M%S" {
                        return DateDetection::FoundValidFormat(parsed_date);
                    } else {
                        return DateDetection::FoundButBadFormat(date_str);
                    }
                }
            }
        }
    }

    DateDetection::NotFound
}
