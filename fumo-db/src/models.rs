use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema;

#[derive(Insertable, Deserialize)]
#[diesel(table_name = schema::fumos)]
pub struct NewFumo {
    pub caption: String,
    pub img: String,
    pub public: bool,
    pub attribution: String,
    pub submitter: String,
    pub involved: Vec<Option<String>>,
}

#[derive(Queryable, Selectable, serde::Serialize)]
#[diesel(table_name = schema::fumos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Fumo {
    pub id: i64,
    pub caption: String,
    pub img: String,
    pub involved: Vec<Option<String>>,
    pub attribution: String,
    pub submitter: String, //Submitters have the following structure: "{platform} {submitter_id}-{submission_id}". platform is a three letter code. Only dsc atm.
}

pub fn is_valid_involvable(fumo: &String) -> bool {
    INVOLVABLE.contains(&fumo.as_str())
}

pub static INVOLVABLE: &[&str] = &["Cirno", "Reimu", "Remilia"];


#[derive(Debug, PartialEq, Queryable, Selectable, Serialize)]
#[diesel(table_name = schema::fumos)]
pub struct APIFumo {
    pub caption: String,
    pub img: String,
    pub attribution: String,
    pub involved: Vec<Option<String>>
}