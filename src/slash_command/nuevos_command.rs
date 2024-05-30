

use crate::pg_database::get_employee_by_ts_range;
use crate::utils::{
    group_employees_by_month::group_employees_by_month, parse_interval::parse_interval,
    response_templates::new_employees_template, ParseDateStrError,
};

use rocket::{form::Form, http::Status, response::status};

#[derive(FromForm, Debug)]
pub struct ListNewsEmployeesCommand {
    pub token: String,
    pub api_app_id: String,
    pub command: String,
    pub text: String,
    pub response_url: String,
}

#[post(
    "/command/nuevos",
    data = "<command>",
    format = "application/x-www-form-urlencoded"
)]
pub fn nuevos_command_route(command: Form<ListNewsEmployeesCommand>) -> status::Custom<String> {
    let parsed = parse_interval(&command.text);
    match parsed {
        Ok((from, to)) => {
            let employees = get_employee_by_ts_range(from, to);

            let employees_by_month = group_employees_by_month(employees);

            let formatted_employees = new_employees_template(from.and_utc().timestamp(), to.and_utc().timestamp(), employees_by_month);

            status::Custom(
                Status::Ok,
                formatted_employees,
            )
        }
        Err(e) => match e {
            ParseDateStrError::Date(invalid) => {
                status::Custom(Status::Ok, format!("Fecha invalida: {}. Escribí /ayuda para ver opciones de formato.", invalid))
            }
            ParseDateStrError::DatePart(invalid) => status::Custom(
                Status::Ok,
                format!(
                    "El fragmento de fecha: {} de {} es inválido. Escribí /ayuda para ver opciones de formato.",
                    invalid, command.text
                ),
            ),
            ParseDateStrError::Interval(invalid) => {
                status::Custom(Status::Ok, format!("El intervalo {} es", invalid))
            }
            ParseDateStrError::NoDate => status::Custom(
                Status::Ok,
                "No se recibió fecha. Escribí /ayuda para ver opciones de formato.".to_string(),
            ),
        },
    }
}