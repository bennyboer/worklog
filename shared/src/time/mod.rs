mod duration_parser;

pub use duration_parser::format_duration;
pub use duration_parser::parse_duration;

/// Get local date time for the passed timestamp.
pub fn get_local_date_time(timestamp_millis: i64) -> chrono::DateTime<chrono::Local> {
    let date_time: chrono::DateTime<chrono::Utc> = chrono::DateTime::from_utc(
        chrono::NaiveDateTime::from_timestamp(timestamp_millis / 1000, 0),
        chrono::Utc,
    );

    // Adjust UTC date time to the local timezone
    chrono::DateTime::from(date_time)
}
