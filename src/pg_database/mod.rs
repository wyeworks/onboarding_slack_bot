use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::models::Employee;
use crate::schema::employees;

pub struct PGDatabase {
    conn: PgConnection,
}

impl PGDatabase {
    pub fn new() -> Self {
        let mut conn = establish_connection();
        PGDatabase { conn }
    }

    pub fn save_employee(&mut self, employee: &Employee) -> Employee {
        let new_employee = Employee {
            id: employee.id,
            email: employee.email.clone(),
            full_name: employee.full_name.clone(),
            country: employee.country.clone(),
            join_date: employee.join_date,
        };

        diesel::insert_into(employees::table)
            .values(&new_employee)
            .returning(Employee::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new post")
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
