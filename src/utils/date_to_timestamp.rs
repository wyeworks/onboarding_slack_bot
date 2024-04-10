use chrono::{NaiveDate, Utc, TimeZone};

pub fn date_to_timestamp(date_str: &str) -> Result<i64, chrono::ParseError> {
    // Parse the date string into a NaiveDate
    let date = NaiveDate::parse_from_str(date_str, "%d/%m/%Y")?;

    // Create a NaiveDateTime at midnight (start of the day)
    let datetime = date.and_hms(0, 0, 0);

    // Assume the date is in UTC for conversion to timestamp
    // If you need a different timezone, adjust accordingly
    let timestamp = Utc.from_utc_datetime(&datetime).timestamp();

    Ok(timestamp)
}