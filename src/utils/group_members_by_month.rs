use std::collections::BTreeMap;

use super::{start_of_month::start_of_month, types::MembersByMonth};

pub fn group_members_by_month(members: Vec<(i64, String)>) -> MembersByMonth {
    let mut members_by_month: MembersByMonth = BTreeMap::new();

    for (ts, id) in members {
        match members_by_month.get(&start_of_month(ts)) {
            Some(members) => {
                let mut members = members.clone();
                members.push(id.clone());
                members_by_month.insert(start_of_month(ts), members);
            }
            None => {
                members_by_month.insert(start_of_month(ts), vec![id]);
            }
        }
    }
    members_by_month
}

#[cfg(test)]
mod test_group_members_by_month {
    use std::collections::BTreeMap;

    use super::group_members_by_month;

    #[test]
    fn test_group_members_by_month() {
        let ts0_2024 = 1706745600; // 2024-02-01 00:00:00 UTC-0
        let ts0_2030 = 1906502400; // 2030-06-01 00:00:00 UTC-0

        let (ts1, member1) = (1707745600, "GHI789"); // 2024-02-12 UTC-0
        let (ts2, member2) = (1708048800, "ABC123"); // 2024-02-16 UTC-0
        let (ts3, member3) = (1908048800, "DEF456"); // 2030-06-18 UTC-0
        let members: Vec<(i64, String)> = vec![
            (ts1, member1.to_string()),
            (ts2, member2.to_string()),
            (ts3, member3.to_string()),
        ];
        let result = group_members_by_month(members);

        let mut expected = BTreeMap::new();
        expected.insert(ts0_2024, vec![member1.to_string(), member2.to_string()]);
        expected.insert(ts0_2030, vec![member3.to_string()]);

        assert_eq!(result, expected);
    }
}
