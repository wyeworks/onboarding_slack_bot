use std::collections::BTreeMap;

pub enum DateRound {
    Ceil,
    Floor,
}

pub type MembersByMonth = BTreeMap<i64, Vec<String>>;
