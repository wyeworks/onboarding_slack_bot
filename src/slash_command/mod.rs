#[cfg(test)]
mod tests;

use crate::pg_database::get_employee_by_ts_range;
use crate::utils::{
    group_employees_by_month::group_employees_by_month, parse_interval::parse_interval,
    response_templates::new_employees_template, ParseDateStrError,
};

use rocket::{form::Form, http::Status, response::status};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "command")]
pub enum SlackCommand {
    ListNewPeople,
}

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
pub fn slash_command_route(command: Form<ListNewsEmployeesCommand>) -> status::Custom<String> {
    println!("slash command: {} {}", command.command, command.text);
    let parsed = parse_interval(&command.text);
    match parsed {
        Ok((from, to)) => {
            let employees = get_employee_by_ts_range(from, to);

            let employees_by_month = group_employees_by_month(employees);

            status::Custom(
                Status::Ok,
                serde_json::to_string(&employees_by_month).unwrap(),
            )
        }
        Err(e) => match e {
            ParseDateStrError::Date(invalid) => {
                status::Custom(Status::Ok, format!("Fecha invalida: {}", invalid))
            }
            ParseDateStrError::DatePart(invalid) => status::Custom(
                Status::Ok,
                format!(
                    "El fragmento de fecha: {} de {} es invalido",
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

#[post("/command/ayuda")]
pub fn help_command_route() -> status::Custom<String> {
    let help_message = "
                    - Para listar nuevos empleados dentro de un rango de fechas específico, escribí `/nuevos <fecha_inicio> <fecha_fin>`.\n\
                    - Podés usar fechas completas (DD/MM/YYYY), mes y año (MM/YYYY) o sólo año (YYYY).";

    status::Custom(Status::Ok, help_message.to_string())
}
