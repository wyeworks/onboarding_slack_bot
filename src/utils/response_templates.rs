use chrono::{Datelike, LocalResult, TimeZone, Utc};

use super::EmployeesByMonth;

const SPANISH_MONTHS: [&str; 12] = [
    "Enero",
    "Febrero",
    "Marzo",
    "Abril",
    "Mayo",
    "Junio",
    "Julio",
    "Agosto",
    "Septiembre",
    "Octubre",
    "Noviembre",
    "Diciembre",
];

fn tag(id: &str) -> String {
    format!("<@{}>", id)
}

fn employee_list(employee_ids: Vec<String>) -> String {
    employee_ids
        .iter()
        .map(|u| format!("- {}", tag(u)))
        .collect::<Vec<String>>()
        .join("\n")
}

fn employee_list_by_month(employees_by_month: EmployeesByMonth) -> String {
    employees_by_month
        .iter()
        .rev()
        .map(|(&month, employees)| {
            let str_month = Utc
                .timestamp_opt(month, 0)
                .map(|d| {
                    let fmt_month = SPANISH_MONTHS.get(d.month0() as usize).unwrap();
                    let y = d.year();
                    format!("{} {}", fmt_month, y)
                })
                .unwrap();

            format!("{}:\n", str_month) + &employee_list(employees.clone())
        })
        .collect::<Vec<String>>()
        .join("\n\n")
}

pub fn new_employees_template(
    from_ts: i64,
    to_ts: i64,
    employees_by_month: EmployeesByMonth,
) -> String {
    let [from, to] = [from_ts, to_ts].map(|d| {
        Utc.timestamp_opt(d, 0)
            .map(|d| d.format("%d/%m/%Y").to_string())
    });

    match (from, to) {
        (LocalResult::Single(from), LocalResult::Single(to)) => {
            let base_template =
                format!("Los que entraron desde el {} hasta el {} son: \n", from, to);
            let employees_by_month_template = employee_list_by_month(employees_by_month);

            base_template + &employees_by_month_template
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
    fn test_employee_list() {
        let employees = vec![
            "ABC123".to_string(),
            "DEF456".to_string(),
            "GHI789".to_string(),
        ];
        let expected = "- <@ABC123>\n- <@DEF456>\n- <@GHI789>";
        assert_eq!(employee_list(employees), expected);
    }

    #[test]
    fn test_employee_list_by_month() {
        let one_week = 604800;
        let jun_2030 = 1906502400;
        let feb_2024 = 1706745600;
        let mut employees_by_month = BTreeMap::new();
        employees_by_month.insert(feb_2024 + one_week, vec!["ABC123".to_string()]);
        employees_by_month.insert(
            jun_2030 + one_week,
            vec!["DEF456".to_string(), "GHI789".to_string()],
        );

        let expected = "Junio 2030:\n- <@DEF456>\n- <@GHI789>\n\nFebrero 2024:\n- <@ABC123>";
        assert_eq!(employee_list_by_month(employees_by_month), expected);
    }

    #[test]
    fn test_new_employees_template() {
        let one_week = 604800;
        let jun_2030 = 1906502400;
        let feb_2024 = 1706745600;
        let mut employees_by_month = BTreeMap::new();
        employees_by_month.insert(feb_2024 + one_week, vec!["ABC123".to_string()]);
        employees_by_month.insert(
            jun_2030 + one_week,
            vec!["DEF456".to_string(), "GHI789".to_string()],
        );

        let from_ts = 1612137600; // 2021-02-01 00:00:00 UTC-0
        let to_ts = 1906502400; // 2030-06-01 00:00:00 UTC-0
        let result = new_employees_template(from_ts, to_ts, employees_by_month);

        let expected = "Los que entraron desde el 01/02/2021 hasta el 01/06/2030 son: \nJunio 2030:\n- <@DEF456>\n- <@GHI789>\n\nFebrero 2024:\n- <@ABC123>";

        assert_eq!(result, expected);
    }
}
