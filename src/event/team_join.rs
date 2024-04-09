use super::{Member, TeamJoinUser};
use crate::database::{get_conn, DatabaseActions};
use chrono::Local;

pub fn handle_team_join(user: TeamJoinUser) {
    let timestamp = Local::now().timestamp();

    let cloned_user = user.clone();
    let member = Member {
        id: user.id,
        email: user.profile.email,
        full_name: user.profile.display_name,
        country: user.tz_label.to_lowercase().replace(" time", ""),
        _raw: cloned_user,
    };

    let mut db = get_conn();
    let _ = db.add_member_to_set(&member.id, timestamp);
    let _ = db.save_member(&member);
}
