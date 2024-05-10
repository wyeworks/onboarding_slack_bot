#[cfg(test)]
mod tests;

use redis::Commands;
use std::{env, str::FromStr};

use crate::event::Employee;

const MEMBER_JOIN_SET_NAME: &str = "member_join_timestamp";
const MEMBER_HASH_NAME: &str = "members";

pub mod db_seeder;

pub struct Database {
    conn: redis::Connection,
}
pub trait DatabaseActions {
    fn add_member_to_set(&mut self, member_id: &str, ts: i64) -> Result<(), String>;
    fn save_member(&mut self, member: &Employee) -> Result<(), String>;
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

    fn save_member(&mut self, member: &Employee) -> Result<(), String> {
        match serde_json::to_string(member) {
            Ok(json_member) => {
                match self.conn.hset::<&str, &str, String, ()>(
                    MEMBER_HASH_NAME,
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
                MEMBER_JOIN_SET_NAME,
                from_ts,
                to_ts,
            ) {
            Ok(r) => match zrange_vec_to_tuple_vec(r) {
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            },
            Err(_) => Err(format!(
                "Failed to get member ids with range: ({}, {})",
                from_ts, to_ts
            )),
        }
    }
}

fn zrange_vec_to_tuple_vec<T: FromStr>(v: Vec<String>) -> Result<Vec<(T, String)>, String>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    v.chunks(2)
        .map(|x| {
            let score = x[1].parse::<T>();
            let value = x[0].to_string();
            println!("score: {:?}", x);
            match score {
                Ok(s) => Ok((s, value)),
                Err(_) => Err("Failed to parse score".to_string()),
            }
        })
        .collect()
}
