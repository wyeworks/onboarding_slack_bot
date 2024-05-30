use rocket::{http::Status, response::status};

#[post("/command/ayuda")]
pub fn ayuda_command_route() -> status::Custom<String> {
    let help_message = "
                    - Para listar nuevos empleados dentro de un rango de fechas específico, escribí `/nuevos <fecha_inicio> <fecha_fin>`.\n\
                    - Podés usar fechas completas (DD/MM/YYYY), mes y año (MM/YYYY) o sólo año (YYYY).";

    status::Custom(Status::Ok, help_message.to_string())
}
