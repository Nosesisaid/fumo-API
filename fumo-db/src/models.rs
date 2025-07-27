use diesel::{prelude::*, serialize};

use crate::schema;


#[derive(Insertable)]
#[diesel(table_name = schema::fumos)]
pub struct NewFumo<'a> {
    pub caption: &'a str,
    pub img: &'a str,
    pub public: bool,
    pub attribution: Option<&'a str>,
    pub submitter: Option<&'a str>,
    pub involved: Option<&'a [Option<String>]>
}

#[derive(Queryable,Selectable, serde::Serialize)]
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

pub fn is_valid_fumo(fumo: &String) ->bool {
    INVOLVABLE.contains(&fumo.as_str())
}

pub static INVOLVABLE: &[&str] = &["Cirno", "Reimu", "Remilia"];