use chrono::{Datelike, NaiveDate, NaiveDateTime};

pub fn start_of_month(ts: i64) -> i64 {
    NaiveDateTime::from_timestamp_opt(ts, 0)
        .map(|d| NaiveDate::from_ymd_opt(d.year(), d.month(), 1).unwrap())
        .map(|d| d.and_hms_opt(0, 0, 0).unwrap())
        .unwrap()
        .and_utc()
        .timestamp()
}

#[cfg(test)]
mod test_start_of_month {
    use super::start_of_month;

    #[test]
    fn should_return_start_of_month() {
        let ts = 1612137600; // 2021-02-01 00:00:00 UTC-0
        let start = start_of_month(ts);
        assert_eq!(start, ts);
    }

    #[test]
    fn should_return_start_of_month_for_different_ts() {
        let ts = 1706745600; // 2024-02-01 00:00:00 UTC-0
        let some_day_of_feb = 1708306735;
        let start = start_of_month(some_day_of_feb);
        assert_eq!(start, ts);
    }
}
