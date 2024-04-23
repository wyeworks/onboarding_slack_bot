pub mod group_members_by_month;
pub mod last_day_of_month;
pub mod load_env;
pub mod parse_date_str;
pub mod parse_interval;
pub mod response_templates;
pub mod start_of_month;

use std::collections::BTreeMap;

pub enum DateRound {
    Ceil,
    Floor,
}

pub type MembersByMonth = BTreeMap<i64, Vec<String>>;

#[derive(Debug)]
pub enum ParseDateStrError {
    DatePart(String),
    Date(String),
    Interval(String),
    NoDate,
}
