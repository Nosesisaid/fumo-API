use diesel::prelude::*;

use crate::database::schema;

#[derive(Queryable,Selectable)]
#[diesel(table_name = schema::fumos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Fumo {
    pub id: i32,
    caption: String,
    img: String,
    involved: Vec<String>,
    attribution: String
}


pub const INVOLVABLE: [&str;3] = ["Cirno", "Reimu", "Remilia"];