use diesel::{prelude::*};

use crate::schema;

#[derive(Queryable,Selectable)]
#[diesel(table_name = schema::fumos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Fumo {
    pub id: i64,
    pub caption: String,
    pub img: String,
    pub involved: Option<Vec<Option<String>>>,
    pub attribution: Option<String>,
    pub submitter: String
}


pub const INVOLVABLE: [&str;3] = ["Cirno", "Reimu", "Remilia"];