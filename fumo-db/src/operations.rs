use core::error;

use crate::models::Fumo;
use crate::models::INVOLVABLE;
use crate::models::NewFumo;
use crate::models::is_valid_involvable;
use crate::schema::fumos::dsl::*;
use diesel::associations::HasTable;
use diesel::dsl::*;
pub use diesel::pg::PgConnection;
use diesel::prelude::*;
pub use diesel::result::QueryResult;

pub fn fumo_count(conn: &mut PgConnection) -> QueryResult<u64> {
    let c: i64 = fumos.select(count(id)).first(conn)?;

    Ok(c as u64)
}

pub fn fumo_count_by(conn: &mut PgConnection, fumo: String) -> QueryResult<u64> {
    if !is_valid_involvable(&fumo) {
        return Err(diesel::result::Error::DeserializationError(
            "Invalid fumo provided".into(),
        ));
    };

    let count: Result<i64, diesel::result::Error> = fumos
        .select(count(id))
        .filter(involved.contains(vec![&fumo]))
        .first(conn);
    match count {
        Ok(c) => Ok(c as u64),
        Err(e) => Err(e),
    }
}

pub fn fetch_fumos(
    conn: &mut PgConnection,
    offset: i64,
    limit: Option<i64>,
) -> QueryResult<Vec<Fumo>> {
    let limit = limit.unwrap_or(15);
    fumos
        .select(Fumo::as_select())
        .limit(limit)
        .offset(offset)
        .load(conn)
}

pub fn add_fumo(conn: &mut PgConnection, fumo_to_add: NewFumo) -> QueryResult<Fumo> {
    if fumo_to_add.involved.as_ref().is_some_and(|invlvd| {
        invlvd
            .iter()
            .filter_map(|element| {
                element.as_ref().and_then(|e| {
                    if INVOLVABLE.contains(&e.as_str()) {
                        Some(e)
                    } else {
                        None
                    }
                })
            })
            .count()
            > 0
    }) {
        return Err(diesel::result::Error::DeserializationError(
            "Invalid involved array provided".into(),
        ));
    }

    insert_into(fumos::table())
        .values(fumo_to_add)
        .returning(Fumo::as_returning())
        .get_result(conn)
}

pub fn fetch_fumos_fby_involved() {
    todo!()
}

pub fn edit_involved(
    conn: &mut PgConnection,
    fumo_unique_id: i64,
    new_involved: Vec<String>,
) -> QueryResult<Fumo> {
    let mut new_involved_typed: Vec<Option<String>> = vec![];

    for element in new_involved {
        if !is_valid_involvable(&element) {
            return Err(diesel::result::Error::QueryBuilderError(
                "Invalid involvable provided".into(),
            ));
        }

        new_involved_typed.push(Some(element));
    }

    let new_involved_typed = Some(new_involved_typed);
    let result = update(fumos)
        .filter(id.eq(fumo_unique_id))
        .set(involved.eq(new_involved_typed))
        .returning(Fumo::as_returning())
        .get_result(conn);

    return result;
}
