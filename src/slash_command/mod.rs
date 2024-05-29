#[cfg(test)]
mod tests;

use crate::{
    database::{get_conn, DatabaseActions},
    utils::{
        group_employees_by_month::group_employees_by_month, parse_interval::parse_interval,
        response_templates::new_employees_template, ParseDateStrError,
    },
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
            let from_ts = from.timestamp();
            let to_ts = to.timestamp();
            println!("from: {}, to: {}", from_ts, to_ts);
            match get_conn().get_employee_id_by_ts_range(from_ts, to_ts) {
                Ok(employees) => {
                    let employees_by_month = group_employees_by_month(employees);
                    let formated_employees =
                        new_employees_template(from_ts, to_ts, employees_by_month);
                    status::Custom(Status::Ok, formated_employees)
                }
                Err(e) => {
                    println!("{}", e);
                    status::Custom(Status::Ok, "Error desconocido".to_string())
                }
            }
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

#[post("/command/ayuda")]
pub fn help_command_route() -> status::Custom<String> {
    let help_message = "
                    - Para listar nuevos empleados dentro de un rango de fechas específico, escribí `/nuevos <fecha_inicio> <fecha_fin>`.\n\
                    - Podés usar fechas completas (DD/MM/YYYY), mes y año (MM/YYYY) o sólo año (YYYY).";

    status::Custom(Status::Ok, help_message.to_string())
}
