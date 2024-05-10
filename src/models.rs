use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Employee {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub published: bool,
}