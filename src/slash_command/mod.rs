#[cfg(test)]
mod tests;

use crate::{
    database::{get_conn, DatabaseActions},
    utils::{
        group_members_by_month::group_members_by_month, parse_interval::parse_interval,
        response_templates::new_members_template, ParseDateStrError,
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
            match get_conn().get_member_id_by_ts_range(from_ts, to_ts) {
                Ok(members) => {
                    let members_by_month = group_members_by_month(members);
                    let formated_members = new_members_template(from_ts, to_ts, members_by_month);
                    status::Custom(Status::Ok, formated_members)
                }
                Err(e) => {
                    println!("{}", e);
                    status::Custom(Status::Ok, "Error desconocido".to_string())
                }
            }
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
        },
    }
}

#[post("/command/help")]
pub fn help_command_route() -> status::Custom<String> {
    let help_message = "
                    - Para listar nuevos empleados dentro de un rango de fechas específico, escribí `/nuevos <fecha_inicio> <fecha_fin>`.\n\
                    - Podés usar fechas completas (DD-MM-YYYY), mes y año (MM-YYYY) o sólo año (YYYY).";

    status::Custom(Status::Ok, help_message.to_string())
}
