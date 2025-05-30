use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::todo)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::todo)]
pub struct NewTodo<'a> {
    pub title: &'a str,
}
