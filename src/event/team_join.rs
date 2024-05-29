use super::TeamJoinUser;
use crate::models::Employee;
use chrono::Local;

pub fn handle_team_join(user: TeamJoinUser) {
    let timestamp = Local::now().timestamp();

    let employee = Employee {
        id: user.id,
        email: user.profile.email,
        full_name: user.profile.display_name,
        country: Some(user.tz_label.to_lowercase().replace(" time", "")),
        join_date: Local::now().naive_utc(),
    };
}
