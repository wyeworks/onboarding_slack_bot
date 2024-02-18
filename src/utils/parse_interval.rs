use chrono::{NaiveDateTime, Utc};

use super::{parse_date_str::parse_date_str, types::DateRound};

pub fn parse_interval(command_text: &str) -> Result<(NaiveDateTime, NaiveDateTime), String> {
    let today = Utc::now().naive_local();

    let lower = command_text.to_lowercase();
    let v = lower.trim().split(' ').collect::<Vec<&str>>();

    match v.len() {
        0 => Ok((today, today)),
        1 => match parse_date_str(v[0], DateRound::Floor) {
            Ok(from) => Ok((from, today)),
            Err(e) => Err(e),
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
        _ => Err("Invalid prompt".to_string()),
    }
}

#[cfg(test)]
mod test_parse_interval {
    #[test]
    fn init() {
        assert_eq!(1, 1);
    }
}
