use chrono::NaiveDateTime;

use super::{parse_date_str::parse_date_str, DateRound, ParseDateStrError};

pub fn parse_interval(
    command_text: &str,
) -> Result<(NaiveDateTime, NaiveDateTime), ParseDateStrError> {
    let lower = command_text.to_lowercase();
    let v = lower
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    match v.len() {
        0 => Err(ParseDateStrError::NoDate),
        1 => match (
            parse_date_str(v[0], DateRound::Floor),
            parse_date_str(v[0], DateRound::Ceil),
        ) {
            (Ok(from), Ok(to)) => Ok((from, to)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        },
        2 => {
            let parsed_from = parse_date_str(v[0], DateRound::Floor);
            let parsed_to = parse_date_str(v[1], DateRound::Ceil);

            match (parsed_from, parsed_to) {
                (Ok(from), Ok(to)) => Ok((from, to)),
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
            }
        }
        _ => Err(ParseDateStrError::Interval(command_text.to_string())),
    }
}

#[cfg(test)]
mod test_parse_interval {
    use chrono::Utc;

    use super::parse_interval;

    #[test]
    fn should_return_from_to_today_tuple_with_one_date() {
        let param = "01/01/2021";
        let param_date = chrono::NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
        let param_date_eod = param_date.and_hms_opt(23, 59, 59).unwrap();
        let param_date_bod = param_date.and_hms_opt(0, 0, 0).unwrap();

        let (from, to) = parse_interval(param).unwrap();

        assert_eq!(from, param_date_bod);
        assert_eq!(to, param_date_eod);
    }

    #[test]
    fn should_return_from_to_today_tuple_with_two_dates() {
        let from = "01/01/2021";
        let to = "02/01/2021";
        let from_date = chrono::NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
        let to_date = chrono::NaiveDate::from_ymd_opt(2021, 1, 2).unwrap();
        let from_date_bod = from_date.and_hms_opt(0, 0, 0).unwrap();
        let to_date_eod = to_date.and_hms_opt(23, 59, 59).unwrap();

        let (from, to) = parse_interval(format!("{} {}", from, to).as_str()).unwrap();

        assert_eq!(from, from_date_bod);
        assert_eq!(to, to_date_eod);
    }
}
