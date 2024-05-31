use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::employees)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Employee {
    pub id: String,
    pub email: String,
    pub full_name: String,
    pub country: Option<String>,
    pub join_date: NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Projects {
    pub id: Uuid,
    pub name: String,
    pub admin_id: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::onboardees)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Onboardees {
    pub project_id: Uuid,
    pub employee_id: String,
    pub onboarding_date: NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::admins)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Admins {
    pub id: String,
}
