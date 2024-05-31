use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::models::Admins;
use crate::models::Employee;
use crate::models::Projects;
use crate::schema::admins;
use crate::schema::employees;
use crate::schema::projects;

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

sql_function! {
    fn lower(x: diesel::sql_types::Text) -> diesel::sql_types::Text;
}

pub fn is_onboarding_admin(user_id: &str) -> bool {
    let conn = &mut establish_connection();
    let results = admins::table
        .filter(admins::id.eq(user_id))
        .load::<Admins>(conn)
        .expect("Error loading admins");

    // Esto debería ser !results.is_empty() pero lo doy vuelta para que retorne true
    // hasta que haya admins en la base de datos
    results.is_empty()
}

pub fn create_project(project: Projects) -> Result<(), String> {
    let conn = &mut establish_connection();

    // Check if project already exists ignoring case
    let results = projects::table
        .filter(lower(projects::name).eq(lower(&project.name)))
        .load::<Projects>(conn)
        .expect("Error loading projects");

    if !results.is_empty() {
        return Err("Project already exists".to_string());
    }

    diesel::insert_into(projects::table)
        .values(&project)
        .execute(conn)
        .expect("Error saving new project");

    Ok(())
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
