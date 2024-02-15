use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use dotenv::dotenv;
use redis::Commands;
use rocket::{
    form::{Form, FromForm},
    http::Status,
    response::status,
    serde::json::Json,
    Build, Config, Rocket,
};
use serde::{Deserialize, Serialize};
use std::{env, fmt::Debug, str::FromStr};
#[macro_use]
extern crate rocket;
extern crate dotenv;
extern crate redis;

fn load_env() {
    fn validate_env_vars() {
        for env_var in vec!["APP_PORT"].iter() {
            match env::var(env_var) {
                Ok(_) => {}
                Err(_) => panic!("{} is not set", env_var),
            }
        }
    }

    match env::var("APP_ENV") {
        Ok(_) => {
            println!("Using environment variables");
        }
        Err(_) => {
            dotenv().ok();
            println!("Using .env file");
        }
    }
    validate_env_vars();
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
enum SlackCallback {
    #[serde(rename = "url_verification")]
    Challenge { token: String, challenge: String },
    #[serde(rename = "event_callback")]
    EventCallback {
        token: String,
        api_app_id: String,
        event: Event,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
enum Event {
    #[serde(rename = "team_join")]
    TeamJoin { user: TeamJoinUser },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct TeamJoinUser {
    id: String,
    profile: TeamJoinUserProfile,
    tz: String,
    tz_label: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct TeamJoinUserProfile {
    email: String,
    display_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Member {
    id: String,
    email: String,
    full_name: String,
    country: String,
    _raw: TeamJoinUser,
}

fn authenticate(token: &str, api_app_id: &str) -> Result<(), ()> {
    let env_slack_token = env::var("SLACK_TOKEN").unwrap();
    let env_slack_api_app_id = env::var("SLACK_API_APP_ID").unwrap();

    if token != env_slack_token || api_app_id != env_slack_api_app_id {
        return Err(());
    }
    Ok(())
}

#[post("/event", data = "<json_callback>", format = "json")]
fn event_route(json_callback: Json<SlackCallback>) -> status::Custom<Json<SlackCallback>> {
    println!("{:?}", json_callback);

    let cb = json_callback.clone().into_inner();

    match cb {
        SlackCallback::Challenge { .. } => println!("Challenge event"),
        SlackCallback::EventCallback {
            event,
            token,
            api_app_id,
        } => {
            if authenticate(&token, &api_app_id).is_ok() {
                handle_event(event)
            } else {
                println!("Authentication failed");
            }
        }
    }

    status::Custom(Status::Ok, json_callback)
}

fn handle_event(event: Event) {
    let mut redis = redis_con();
    match event {
        Event::TeamJoin { user } => {
            let timestamp = Local::now().timestamp();
            let member = Member {
                id: user.id.clone(),
                email: user.profile.email.clone(),
                full_name: user.profile.display_name.clone(),
                country: user.tz_label.to_lowercase().replace(" time", ""),
                _raw: user,
            };
            let stringified_member = serde_json::to_string(&member);
            match stringified_member {
                Ok(s) => {
                    let _ = redis.zadd::<&str, i64, String, ()>(
                        "member_join_timestamp",
                        member.id.clone(),
                        timestamp,
                    );
                    let _ = redis.hset::<&str, &str, String, ()>("members", &member.id, s);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "command")]
enum SlackCommand {
    ListNewPeople,
}

#[derive(FromForm, Debug)]
struct ListNewsEmployeesCommand {
    token: String,
    api_app_id: String,
    command: String,
    text: String,
    response_url: String,
}

/* --- Slash commands --- */
#[post(
    "/command/nuevos",
    data = "<command>",
    format = "application/x-www-form-urlencoded"
)]
fn slash_command_route(command: Form<ListNewsEmployeesCommand>) -> status::Custom<String> {
    let mut redis = redis_con();
    let parsed = parse_interval(&command.text);
    match parsed {
        Ok((from, to)) => {
            let from_ts = from.timestamp();
            let to_ts = to.timestamp();
            println!("from: {}, to: {}", from_ts, to_ts);
            let members = redis.zrangebyscore_withscores::<&str, i64, i64, Vec<String>>(
                "member_join_timestamp",
                from_ts,
                to_ts,
            );
            match members {
                Ok(members) => {
                    let formated_members = members
                        .chunks(2)
                        .map(|m| format!("<@{}>", m[0]))
                        .collect::<Vec<String>>()
                        .join(", ");
                    status::Custom(Status::Ok, formated_members)
                }
                Err(e) => {
                    println!("{}", e);
                    status::Custom(Status::Ok, "error".to_string())
                }
            }
        }
        Err(e) => {
            println!("{}", e);
            status::Custom(Status::Ok, "error".to_string())
        }
    }
}

fn parse_interval(command_text: &str) -> Result<(NaiveDateTime, NaiveDateTime), String> {
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

enum DateRound {
    Ceil,
    Floor,
}

fn last_day_of_month(year: i32, month: u32) -> Option<NaiveDate> {
    if !(1..=12).contains(&month) {
        return None;
    }

    let now = Local::now().date_naive();
    let next_jan_first = NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap_or(now);

    NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or(next_jan_first)
        .pred_opt()
}

fn parse_date_str(date_str: &str, round: DateRound) -> Result<NaiveDateTime, String> {
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

/* --- Misc --- */
fn redis_con() -> redis::Connection {
    let redis_host = env::var("REDIS_HOSTNAME").unwrap();
    let redis_password = env::var("REDIS_PASSWORD").unwrap();
    let redis_uri_scheme = env::var("REDIS_URI_SCHEME").unwrap();

    let redis_conn_url = format!("{}://:{}@{}", redis_uri_scheme, redis_password, redis_host);
    redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}

fn init_rocket() -> Rocket<Build> {
    let config = Config {
        port: env::var("APP_PORT")
            .unwrap()
            .parse()
            .expect("APP_PORT is not a valid number"),
        ..Config::debug_default()
    };

    rocket::build().configure(&config).mount(
        env::var("APP_BASE_ROUTE").unwrap(),
        routes![event_route, slash_command_route],
    )
}

#[launch]
fn init() -> _ {
    load_env();
    init_rocket()
}

#[cfg(test)]
mod test_parse_date_str {
    use crate::{parse_date_str, DateRound};
    use chrono::{Datelike, NaiveDate};

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

#[cfg(test)]
mod test_last_day_of_month {
    use crate::last_day_of_month;
    use chrono::Datelike;

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
            assert!(last_day.is_none());
        }
    }
}

#[cfg(test)]
mod test_parse_interval {}
