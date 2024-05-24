// @generated automatically by Diesel CLI.

diesel::table! {
    employees (id) {
        id -> Varchar,
        email -> Varchar,
        full_name -> Varchar,
        country -> Nullable<Varchar>,
        join_date -> Timestamp,
    }
}

diesel::table! {
    onboardees (project_id, employee_id) {
        project_id -> Uuid,
        employee_id -> Varchar,
        onboarding_date -> Timestamp,
    }
}

diesel::table! {
    projects (id) {
        id -> Uuid,
        name -> Varchar,
        admin_id -> Varchar,
    }
}

diesel::joinable!(onboardees -> employees (employee_id));
diesel::joinable!(onboardees -> projects (project_id));
diesel::joinable!(projects -> employees (admin_id));

diesel::allow_tables_to_appear_in_same_query!(employees, onboardees, projects,);
