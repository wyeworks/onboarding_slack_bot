use std::collections::BTreeMap;

use crate::models::Employee;

use super::{start_of_month::start_of_month, EmployeesByMonth};

pub fn group_employees_by_month(employees: Vec<Employee>) -> EmployeesByMonth {
    let mut employees_by_month: EmployeesByMonth = BTreeMap::new();

    for employee in employees {
        let id = employee.id.clone();
        let ts = employee.join_date.timestamp();

        match employees_by_month.get(&start_of_month(ts)) {
            Some(employees) => {
                let mut employees = employees.clone();
                employees.push(id);
                employees_by_month.insert(start_of_month(ts), employees);
            }
            None => {
                employees_by_month.insert(start_of_month(ts), vec![id]);
            }
        }
    }
    employees_by_month
}

#[cfg(test)]
mod test_group_employees_by_month {
    use std::collections::BTreeMap;

    use super::group_employees_by_month;

    #[test]
    fn test_group_employees_by_month() {
        let ts0_2024 = 1706745600; // 2024-02-01 00:00:00 UTC-0
        let ts0_2030 = 1906502400; // 2030-06-01 00:00:00 UTC-0

        let (ts1, employee1) = (1707745600, "GHI789"); // 2024-02-12 UTC-0
        let (ts2, employee2) = (1708048800, "ABC123"); // 2024-02-16 UTC-0
        let (ts3, employee3) = (1908048800, "DEF456"); // 2030-06-18 UTC-0
        let employees: Vec<(i64, String)> = vec![
            (ts1, employee1.to_string()),
            (ts2, employee2.to_string()),
            (ts3, employee3.to_string()),
        ];
        // let result = group_employees_by_month(employees);

        let mut expected = BTreeMap::new();
        expected.insert(ts0_2024, vec![employee1.to_string(), employee2.to_string()]);
        expected.insert(ts0_2030, vec![employee3.to_string()]);

        // assert_eq!(result, expected);
    }
}
