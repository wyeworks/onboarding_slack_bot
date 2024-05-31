use rocket::{form::Form, http::Status, response::status};

use crate::{
    models::Projects,
    pg_database::{create_project, is_onboarding_admin},
    slash_command::types::SlackCommand,
    utils::parse_slack_id::parse_slack_id,
};

use uuid::Uuid;

#[post(
    "/command/nuevo_proyecto",
    data = "<command>",
    format = "application/x-www-form-urlencoded"
)]
pub fn nuevo_proyecto_route(command: Form<SlackCommand>) -> status::Custom<String> {
    let user_id = &command.user_id;

    if !is_onboarding_admin(user_id) {
        return status::Custom(
            Status::Ok,
            "No tenés permisos de administrador para realizar esta acción.".to_string(),
        );
    }
    let clean_command = command.text.chars().filter(|c| c != &'*' && c != &'_' && c != &'~').collect::<String>();
    
    let args = clean_command.split_whitespace().collect::<Vec<&str>>();
    let mut project_name_vec = Vec::new();

    let mut project_admin = String::new();

    for arg in args {
        if arg.starts_with("<@") {
            project_admin = parse_slack_id(arg);
        } else {
            project_name_vec.push(arg);
        }
    }

    let project_name = project_name_vec.join(" ");
    let project_name = project_name.trim();
    let project_id = Uuid::new_v4();
    let new_project = Projects {
        id: project_id,
        name: project_name.to_string(),
        admin_id: project_admin.to_string(),
    };

    match create_project(new_project) {
        Ok(_) => status::Custom(Status::Ok, "Proyecto creado correctamente".to_string()),
        Err(e) => status::Custom(Status::Ok, e),
    }
}
