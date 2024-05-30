use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::models::Employee;
use crate::schema::employees;

pub mod db_seeder;

pub fn save_employee(employee: &Employee) -> Employee {
    let new_employee = Employee {
        id: employee.id.clone(),
        email: employee.email.clone(),
        full_name: employee.full_name.clone(),
        country: employee.country.clone(),
        join_date: employee.join_date,
    };

    let conn = &mut establish_connection();
    diesel::insert_into(employees::table)
        .values(&new_employee)
        .returning(Employee::as_returning())
        .get_result(conn)
        .expect("Error saving new employee")
}

pub fn get_employee_by_ts_range(from_ts: NaiveDateTime, to_ts: NaiveDateTime) -> Vec<Employee> {
    let conn = &mut establish_connection();
    employees::table
        .filter(
            employees::join_date
                .ge(from_ts)
                .and(employees::join_date.le(to_ts)),
        )
        .load::<Employee>(conn)
        .expect("Error loading employees")
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
