use crate::database::get_conn;
use crate::database::{Database, DatabaseActions};
use crate::event::Member;
use chrono::NaiveDate;
use core::panic;
use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Debug, Deserialize)]
struct SeedMember {
    id: String,
    name: String,
    email: String,
    date: String,
    country: String,
}

fn load_seed_members(file_path: &str) -> Result<Vec<SeedMember>, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path)?;
    let seed_members: Vec<SeedMember> = serde_json::from_str(&file_content)?;
    Ok(seed_members)
}

fn to_member(seed_member: &SeedMember) -> Member {
    Member {
        id: seed_member.id.clone(),
        email: seed_member.email.clone(),
        full_name: seed_member.name.clone(),
        country: seed_member.country.clone(),
        date: seed_member.date.clone(),
    }
}
pub fn seed_database(file_path: &str) -> Result<(), Box<dyn Error>> {
    let seed_members = load_seed_members(file_path)?;
    let mut database: Database = get_conn();

    for seed_member in seed_members {
        let member = to_member(&seed_member);
        database.save_member(&member)?;

        let date = NaiveDate::parse_from_str(&seed_member.date, "%d/%m/%Y")?;
        // let timestamp = date.and_hms_opt(0, 0, 0).timestamp();
        let timestamp = match date.and_hms_opt(0, 0, 0) {
            Some(timestamp) => timestamp.timestamp(),
            None => panic!("Failed to convert date to timestamp"),
        };

        database.add_member_to_set(&member.id, timestamp)?;
    }

    Ok(())
}
