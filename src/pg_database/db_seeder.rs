use crate::models::Employee;
use crate::pg_database::save_employee;
use chrono::NaiveDateTime;
use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Debug, Deserialize)]
struct SeedEmployee {
    id: String,
    name: String,
    email: String,
    date: i64,
    country: String,
}

fn load_seed_employees(file_path: &str) -> Result<Vec<SeedEmployee>, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path)?;
    let seed_employees: Vec<SeedEmployee> = serde_json::from_str(&file_content)?;
    Ok(seed_employees)
}

fn to_employee(seed_employee: &SeedEmployee) -> Employee {
    Employee {
        id: seed_employee.id.clone(),
        email: seed_employee.email.clone(),
        full_name: seed_employee.name.clone(),
        country: Some(seed_employee.country.clone()),
        join_date: NaiveDateTime::from_timestamp_opt(seed_employee.date, 0).unwrap(),
    }
}
pub fn seed_database(file_path: &str) -> Result<(), Box<dyn Error>> {
    let seed_employees = load_seed_employees(file_path)?;

    for seed_employee in seed_employees {
        let employee = to_employee(&seed_employee);

        save_employee(&employee);
    }

    Ok(())
}
