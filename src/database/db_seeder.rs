use crate::database::get_conn;
use crate::database::{Database, DatabaseActions};
use crate::event::Employee;
use chrono::NaiveDate;
use core::panic;
use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Debug, Deserialize)]
struct SeedEmployee {
    id: String,
    name: String,
    email: String,
    date: String,
    country: String,
}

fn load_seed_employees(file_path: &str) -> Result<Vec<SeedEmployee>, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path)?;
    let seed_members: Vec<SeedEmployee> = serde_json::from_str(&file_content)?;
    Ok(seed_members)
}

fn to_employee(seed_employee: &SeedEmployee) -> Employee {
    Employee {
        id: seed_employee.id.clone(),
        email: seed_employee.email.clone(),
        full_name: seed_employee.name.clone(),
        country: seed_employee.country.clone(),
        date: seed_employee.date.clone(),
    }
}
pub fn seed_database(file_path: &str) -> Result<(), Box<dyn Error>> {
    let seed_employees = load_seed_employees(file_path)?;
    let mut database: Database = get_conn();

    for seed_employee in seed_employees {
        let member = to_employee(&seed_employee);
        database.save_member(&member)?;

        let date = NaiveDate::parse_from_str(&seed_employee.date, "%d/%m/%Y")?;
        // let timestamp = date.and_hms_opt(0, 0, 0).timestamp();
        let timestamp = match date.and_hms_opt(0, 0, 0) {
            Some(timestamp) => timestamp.timestamp(),
            None => panic!("Failed to convert date to timestamp"),
        };

        database.add_member_to_set(&member.id, timestamp)?;
    }

    Ok(())
}
