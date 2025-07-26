pub mod database;


use std::env;

use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use dotenvy::dotenv;
use axum::{
    extract::State, routing::{get, post}, response::Json, Router
};

use crate::database::{ create_pool, models::Fumo, DbPool};



#[tokio::main]
async fn main() {

    dotenv().ok();

    let pool = create_pool();


    let API_TOKENS: Vec<&str>  = env::var("API_TOKENS").unwrap_or("".to_string()).split_whitespace().collect();

    tracing_subscriber::fmt::init();

    let app: Router<DbPool> = Router::new()
    .route("/",get(root))
    .merge(admin())
    .merge(fumo())
    .with_state(pool);


    println!("Hello, world!");


    todo!("Actually serve the router")
}


fn admin()-> Router<DbPool> {
    todo!("all the database editing stuff")
}

    
   //Test function. The DB never should return everything unpaginated 
    async fn list_all(
        State(pool): State<DbPool> 
    ) -> Json<Vec<Fumo>> {
        use crate::database::schema::fumos::dsl::*;

        let mut conn = pool.get().expect("Failed getting a connection");

        let results = fumos.select(Fumo::as_select()).load::<Fumo>(&mut conn).expect("Failed to fetch all the fumos");

        Json(results)
    }

fn fumo(State(pool): State<DbPool>) -> Router<DbPool> {



    //todo!("All the reading stuff")
    Router::new().route("/list_all",get(list_all)).with_state(pool)
    
}


async fn root() -> &'static str {
    "Welcome to the fumo-API. Learn more at https://github.com/nosesisaid/fumo-api"
}





