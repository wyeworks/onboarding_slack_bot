use chrono::{Local, NaiveDate};

use super::ParseDateStrError;

pub fn last_day_of_month(year: i32, month: u32) -> Result<NaiveDate, ParseDateStrError> {
    if !(1..=12).contains(&month) {
        return Err(ParseDateStrError::DatePart(month.to_string()));
    }

    let now = Local::now().date_naive();
    let next_jan_first = NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap_or(now);

    Ok(NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or(next_jan_first)
        .pred_opt()
        .unwrap())
}

#[cfg(test)]
mod test_last_day_of_month {
    use chrono::Datelike;

    use crate::utils::last_day_of_month::last_day_of_month;

    #[test]
    fn should_return_last_day_of_the_given_month() {
        for (year, month, expected) in [
            (2024, 2, 29), // leap year
            (2023, 2, 28), // non-leap year
            (2023, 4, 30),
            (2023, 6, 30),
            (2023, 9, 30),
            (2023, 11, 30),
        ] {
            let last_day = last_day_of_month(year, month).unwrap();
            assert_eq!(last_day.day(), expected);
        }
    }

    #[test]
    fn returns_none_if_month_is_invalid() {
        let invalid_months = vec![0, 13];
        for month in invalid_months {
            let last_day = last_day_of_month(2023, month);
            assert!(last_day.is_err());
        }
    }
}
