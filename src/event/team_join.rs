use super::{Employee, TeamJoinUser};
use crate::database::{get_conn, DatabaseActions};
use chrono::Local;

pub fn handle_team_join(user: TeamJoinUser) {
    let timestamp = Local::now().timestamp();

    let employee = Employee {
        id: user.id,
        email: user.profile.email,
        full_name: user.profile.display_name,
        country: user.tz_label.to_lowercase().replace(" time", ""),
        date: Local::now().format("%d-%m-%Y").to_string(),
    };

    let mut db = get_conn();
    let _ = db.add_employee_to_set(&employee.id, timestamp);
    let _ = db.save_employee(&employee);
}
