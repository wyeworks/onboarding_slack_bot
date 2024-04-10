use super::{Member, TeamJoinUser};
use crate::database::{get_conn, DatabaseActions};
use chrono::Local;

pub fn handle_team_join(user: &TeamJoinUser) {
    let timestamp = Local::now().timestamp();
    let member = Member {
        id: user.id.clone(),
        email: user.profile.email.clone(),
        full_name: user.profile.display_name.clone(),
        date: Local::now().format("%d/%m/%Y").to_string(),
        country: user.tz_label.to_lowercase().replace(" time", ""),
        _raw: (*user).clone(),
    };

    let mut db = get_conn();
    let _ = db.add_member_to_set(&member.id, timestamp);
    let _ = db.save_member(&member);
}
