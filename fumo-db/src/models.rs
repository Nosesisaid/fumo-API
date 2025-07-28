use diesel::{prelude::*};
use serde::Deserialize;

use crate::schema;

#[derive(Insertable, Deserialize)]
#[diesel(table_name = schema::fumos)]
pub struct NewFumo {
    pub caption: String,
    pub img: String,
    pub public: bool,
    pub attribution: Option<String>,
    pub submitter: Option<String>,
    pub involved: Option<Vec<Option<String>>>,
}

#[derive(Queryable, Selectable, serde::Serialize)]
#[diesel(table_name = schema::fumos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Fumo {
    pub id: i64,
    pub caption: String,
    pub img: String,
    pub involved: Option<Vec<Option<String>>>,
    pub attribution: Option<String>,
    pub submitter: String,
}

pub fn is_valid_involvable(fumo: &String) -> bool {
    INVOLVABLE.contains(&fumo.as_str())
}

pub static INVOLVABLE: &[&str] = &["Cirno", "Reimu", "Remilia"];
