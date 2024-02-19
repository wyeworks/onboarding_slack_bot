#[cfg(test)]
mod tests;

use crate::event::types::Member;
use redis::Commands;
use std::{env, str::FromStr};

const MEMBER_JOIN_SET_NAME: &str = "member_join_timestamp";

pub struct Database {
    conn: redis::Connection,
}
pub trait DatabaseActions {
    fn add_member_to_set(&mut self, member_id: &str, ts: i64) -> Result<(), String>;
    fn save_member(&mut self, member: &Member) -> Result<(), String>;
    fn get_member_id_by_ts_range(
        &mut self,
        from_ts: i64,
        to_ts: i64,
    ) -> Result<Vec<(i64, String)>, String>;
}

pub fn get_conn() -> Database {
    let redis_host = env::var("REDIS_HOSTNAME").unwrap();
    let redis_password = env::var("REDIS_PASSWORD").unwrap();
    let redis_uri_scheme = env::var("REDIS_URI_SCHEME").unwrap();

    let redis_conn_url = format!("{}://:{}@{}", redis_uri_scheme, redis_password, redis_host);
    let conn = redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis");

    Database { conn }
}

impl DatabaseActions for Database {
    fn add_member_to_set(&mut self, member_id: &str, ts: i64) -> Result<(), String> {
        match self.conn.zadd::<&str, i64, String, ()>(
            MEMBER_JOIN_SET_NAME,
            member_id.to_string(),
            ts,
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!(
                "Failed to add member: {} and ts: {} to set",
                member_id, ts
            )),
        }
    }

    fn save_member(&mut self, member: &Member) -> Result<(), String> {
        match serde_json::to_string(member) {
            Ok(json_member) => {
                match self.conn.hset::<&str, &str, String, ()>(
                    "members",
                    &member.id,
                    json_member.clone(),
                ) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format!("Failed to save member: {}", json_member)),
                }
            }
            Err(_) => Err("Failed to convert member to JSON string".to_string()),
        }
    }

    fn get_member_id_by_ts_range(
        &mut self,
        from_ts: i64,
        to_ts: i64,
    ) -> Result<Vec<(i64, String)>, String> {
        match self
            .conn
            .zrangebyscore_withscores::<&str, i64, i64, Vec<String>>(
                "member_join_timestamp",
                from_ts,
                to_ts,
            ) {
            Ok(r) => {
                let x = r
                    .chunks(2)
                    .map(|x| (FromStr::from_str(&x[1]).unwrap(), x[0].to_string()))
                    .collect();

                Ok(x)
            }
            Err(_) => Err(format!(
                "Failed to get member ids with range: ({}, {})",
                from_ts, to_ts
            )),
        }
    }
}
