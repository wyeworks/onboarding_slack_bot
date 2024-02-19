use chrono::{LocalResult, TimeZone, Utc};

use super::types::MembersByMonth;

fn tag(id: &str) -> String {
    format!("<@{}>", id)
}

fn member_list(member_ids: Vec<String>) -> String {
    member_ids
        .iter()
        .map(|u| format!("- {}", tag(u)))
        .collect::<Vec<String>>()
        .join("\n")
}

fn member_list_by_month(members_by_month: MembersByMonth) -> String {
    members_by_month
        .iter()
        .map(|(&month, members)| {
            let str_month = Utc
                .timestamp_opt(month, 0)
                .map(|d| d.format("%B %Y").to_string())
                .unwrap();

            format!("{}:\n", str_month) + &member_list(members.clone())
        })
        .collect::<Vec<String>>()
        .join("\n\n")
}

pub fn new_members_template(from_ts: i64, to_ts: i64, members_by_month: MembersByMonth) -> String {
    let [from, to] = [from_ts, to_ts].map(|d| {
        Utc.timestamp_opt(d, 0)
            .map(|d| d.format("%d/%m/%Y").to_string())
    });

    match (from, to) {
        (LocalResult::Single(from), LocalResult::Single(to)) => {
            let base_template =
                format!("Los que entraron desde el {} hasta el {} son: \n", from, to);
            let members_by_month_template = member_list_by_month(members_by_month);

            base_template + &members_by_month_template
        }
        _ => "Invalid 'from' or 'to' timestamps".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn test_tag() {
        assert_eq!(tag("ABC123"), "<@ABC123>");
    }

    #[test]
    fn test_member_list() {
        let members = vec![
            "ABC123".to_string(),
            "DEF456".to_string(),
            "GHI789".to_string(),
        ];
        let expected = "- <@ABC123>\n- <@DEF456>\n- <@GHI789>";
        assert_eq!(member_list(members), expected);
    }

    #[test]
    fn test_member_list_by_month() {
        let one_week = 604800;
        let jun_2030 = 1906502400;
        let feb_2024 = 1706745600;
        let mut members_by_month = BTreeMap::new();
        members_by_month.insert(feb_2024 + one_week, vec!["ABC123".to_string()]);
        members_by_month.insert(
            jun_2030 + one_week,
            vec!["DEF456".to_string(), "GHI789".to_string()],
        );

        let expected = "February 2024:\n- <@ABC123>\n\nJune 2030:\n- <@DEF456>\n- <@GHI789>";
        assert_eq!(member_list_by_month(members_by_month), expected);
    }

    #[test]
    fn test_new_members_template() {
        let one_week = 604800;
        let jun_2030 = 1906502400;
        let feb_2024 = 1706745600;
        let mut members_by_month = BTreeMap::new();
        members_by_month.insert(feb_2024 + one_week, vec!["ABC123".to_string()]);
        members_by_month.insert(
            jun_2030 + one_week,
            vec!["DEF456".to_string(), "GHI789".to_string()],
        );

        let from_ts = 1612137600; // 2021-02-01 00:00:00 UTC-0
        let to_ts = 1906502400; // 2030-06-01 00:00:00 UTC-0
        let result = new_members_template(from_ts, to_ts, members_by_month);

        let expected = "Los que entraron desde el 01/02/2021 hasta el 01/06/2030 son: \nFebruary 2024:\n- <@ABC123>\n\nJune 2030:\n- <@DEF456>\n- <@GHI789>";
        assert_eq!(result, expected);
    }
}
