use std::str::FromStr;

use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime};

use super::{last_day_of_month::last_day_of_month, types::DateRound};

pub fn parse_date_str(date_str: &str, round: DateRound) -> Result<NaiveDateTime, String> {
    let time = match round {
        DateRound::Ceil => NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
        DateRound::Floor => NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
    };

    let date_parts = date_str.split('/').collect::<Vec<&str>>();
    let parsed_date = match date_parts.len() {
        3 => {
            let (day, month, year) = (
                FromStr::from_str(date_parts[0]),
                FromStr::from_str(date_parts[1]),
                FromStr::from_str(date_parts[2]),
            );

            match (day, month, year) {
                (Ok(day), Ok(month), Ok(year)) => {
                    let d = NaiveDate::from_ymd_opt(year, month, day);
                    d.map(|d| NaiveDateTime::new(d, time))
                }
                _ => None,
            }
        }
        2 => {
            let (month, year) = (
                FromStr::from_str(date_parts[0]),
                FromStr::from_str(date_parts[1]),
            );
            match (year, month) {
                (Ok(year), Ok(month)) => {
                    let day = match round {
                        DateRound::Ceil => last_day_of_month(year, month).unwrap().day(),
                        DateRound::Floor => 1,
                    };
                    let d = NaiveDate::from_ymd_opt(year, month, day);
                    Some(NaiveDateTime::new(d.unwrap(), time))
                }
                _ => None,
            }
        }
        1 => {
            let year = FromStr::from_str(date_parts[0]);

            match year {
                Ok(year) => {
                    let month = match round {
                        DateRound::Ceil => 12,
                        DateRound::Floor => 1,
                    };
                    let day = match round {
                        DateRound::Ceil => last_day_of_month(year, month).unwrap().day(),
                        DateRound::Floor => 1,
                    };
                    let d = NaiveDate::from_ymd_opt(year, month, day);
                    Some(NaiveDateTime::new(d.unwrap(), time))
                }
                _ => None,
            }
        }
        _ => None,
    };
    match parsed_date {
        Some(d) => Ok(d),
        None => Err("Invalid date format, doesn't match any of the supported formats".to_string()),
    }
}

#[cfg(test)]
mod test_parse_date_str {
    use chrono::{Datelike, NaiveDate};

    use crate::utils::{parse_date_str::parse_date_str, types::DateRound};

    fn eod_hms_opt(date: NaiveDate) -> Option<chrono::prelude::NaiveDateTime> {
        date.and_hms_opt(23, 59, 59)
    }
    fn bod_hms_opt(date: NaiveDate) -> Option<chrono::prelude::NaiveDateTime> {
        date.and_hms_opt(0, 0, 0)
    }

    #[test]
    fn should_return_eoy_given_only_a_year_and_ceil() {
        let year = chrono::Utc::now().year();
        let year_str = &year.to_string();
        let eoy = chrono::NaiveDate::from_ymd_opt(year, 12, 31)
            .and_then(eod_hms_opt)
            .unwrap();

        let d = parse_date_str(year_str, DateRound::Ceil);

        assert_eq!(d.unwrap(), eoy);
    }

    #[test]
    fn should_return_first_day_of_year_given_a_year_and_floor() {
        let year = chrono::Utc::now().year();
        let year_str = &year.to_string();
        let jan1 = chrono::NaiveDate::from_ymd_opt(year, 1, 1)
            .and_then(bod_hms_opt)
            .unwrap();

        let d = parse_date_str(year_str, DateRound::Floor);

        assert_eq!(d.unwrap(), jan1);
    }

    #[test]
    fn should_return_first_day_of_month_given_month_year_and_floor() {
        let year = 2024;
        let month = 2;
        let date_str = format!("{}/{}", month, year);
        let feb_1st = chrono::NaiveDate::from_ymd_opt(year, month, 1)
            .and_then(bod_hms_opt)
            .unwrap();

        let d = parse_date_str(&date_str, DateRound::Floor);

        assert_eq!(d.unwrap(), feb_1st);
    }

    #[test]
    fn should_return_last_day_of_month_given_month_year_and_ceil() {
        let year = 2024;
        let month = 2;
        let date_str = format!("{}/{}", month, year);
        let feb_29 = chrono::NaiveDate::from_ymd_opt(year, month, 29)
            .and_then(eod_hms_opt)
            .unwrap();

        let d = parse_date_str(&date_str, DateRound::Ceil);

        assert_eq!(d.unwrap(), feb_29);
    }

    #[test]
    fn should_return_same_day_given_a_full_date() {
        let day = 3;
        let month = 11;
        let year = 1997;

        let date = chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap();
        let date_str = &date.format("%d/%m/%Y").to_string();

        let bod = bod_hms_opt(date).unwrap();
        let eod = eod_hms_opt(date).unwrap();

        let res_bod = parse_date_str(date_str, DateRound::Floor);
        let res_eod = parse_date_str(date_str, DateRound::Ceil);

        assert_eq!(res_bod.unwrap(), bod);
        assert_eq!(res_eod.unwrap(), eod);
    }

    #[test]
    fn should_err_on_invalid_input() {
        let invalid_inputs = [
            "",          // generic invalid
            " ",         // generic invalid
            "a",         // generic invalid
            "1/2/3/4",   // too many parts
            "1/2/3/4/5", // too many parts
            "30/2/2024", // feb is never 30
            "29/2/2023", // 2023 is not a leap year
            "31/4/2023", // april has 30 days
        ];

        for input in invalid_inputs {
            let res = parse_date_str(input, DateRound::Floor);
            assert!(res.is_err());
        }
    }
}
