use crate::models::APIFumo;
use crate::models::Fumo;
use crate::models::NewFumo;
use crate::models::is_valid_involvable;
use crate::schema::fumos::dsl::*;
use diesel::associations::HasTable;
use diesel::dsl::*;
pub use diesel::pg::PgConnection;
use diesel::prelude::*;
pub use diesel::result::QueryResult;
use diesel::sql_types::Integer;

pub fn fumo_count(conn: &mut PgConnection, include_not_public: bool) -> QueryResult<u64> {
    
    let c = fumos.select(count(id));
    if !include_not_public {
        let a: i64 = c.filter(public.eq(true)).first(conn)?;
        return Ok(a as u64)
    }

    let a: i64 = c.first(conn)?;
    Ok( a as u64)
}

pub fn fumo_count_by(conn: &mut PgConnection, fumo: String, include_not_public: bool) -> QueryResult<u64> {
    if !is_valid_involvable(&fumo) {
        return Err(diesel::result::Error::DeserializationError(
            "Invalid fumo provided".into(),
        ));
    };

    let count= fumos
        .select(count(id))
        .filter(involved.contains(vec![&fumo]));

        if !include_not_public {
            return match count.first::<i64>(conn) {
                Ok(c) => Ok(c as u64),
                Err(e) => Err(e)
            }
        }

        

        match count.first::<i64>(conn) {
        Ok(c) => Ok(c as u64),
        Err(e) => Err(e),
    }
}

pub fn fetch_fumos(
    conn: &mut PgConnection,
    offset: i64,
    limit: Option<i64>,
    include_not_public: bool,
) -> QueryResult<Vec<APIFumo>> {
    let limit = limit.unwrap_or(15);
    let mut query = fumos
        .select(APIFumo::as_select())
        .limit(limit)
        .offset(offset);

        if !include_not_public {
            return query.filter(public.eq(true)).load(conn)
        }

        query.load(conn)

}

pub fn add_fumo(conn: &mut PgConnection, fumo_to_add: NewFumo) -> QueryResult<Fumo> {
    if fumo_to_add.involved
            .iter()
            .any(|f| {
                f.is_none()||  (f.is_some() && !is_valid_involvable(f.as_ref().unwrap()))
})
    {
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

    let new_involved_typed = new_involved_typed;
    let result = update(fumos)
        .filter(id.eq(fumo_unique_id))
        .set(involved.eq(new_involved_typed))
        .returning(Fumo::as_returning())
        .get_result(conn);

    return result;
}

pub fn get_random(
    conn: &mut PgConnection,
    fumo: Option<String>,
    include_not_public: bool
) -> QueryResult<APIFumo>{

    let mut query =    fumos.select(APIFumo::as_select()).order_by(sql::<Integer>("RANDOM()")).limit(1).into_boxed();

    if  !include_not_public {
        query = query.filter(public.eq(true));
    }

    if let Some(fumo) = fumo {
        query = query.filter(involved.contains(vec![fumo]));
    } 

    query.first(conn)
}