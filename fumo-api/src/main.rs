

use std::{env, net::SocketAddr};

use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use dotenvy::dotenv;
use axum::{
    extract::State, http::StatusCode, response::{IntoResponse, Json}, routing::{get, post}, Router
};
use fumo_db::{create_pool, models::Fumo, DbPool};


#[derive(Clone)]
struct AppState {
    db: DbPool,
    api_tokens: Vec<String>
}

#[tokio::main]
async fn main() {

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let api_tokens: Vec<String>  = env::var("API_TOKENS").unwrap_or("".to_string()).split_whitespace().map(|s| s.into()).collect();
    let port: u16 = env::var("FUMO_API_PORT").unwrap_or("3000".to_string()).parse().unwrap();


    let db_pool = create_pool(database_url);

    let api_state = AppState {
        db: db_pool,
        api_tokens
    };


    tracing_subscriber::fmt::init();

    let app= Router::new()
    .route("/",get(root))
   // .merge(admin())
    .merge(fumo())
    .with_state(api_state);


    println!("Hello, world!");


    let address = SocketAddr::from(([127,0,0,1],port));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


fn admin()-> Router<AppState> {
    todo!("all the database editing stuff")
}

    
   //Test function. The DB never should return everything unpaginated 
    async fn list_all(
        State(state): State<AppState> 
    ) -> impl IntoResponse {
        let mut conn = state.db.get().expect("Failed getting a connection");

        let results = fumo_db::operations::fetch_fumos(&mut conn, 0, None);

        if let Ok(res)= results  {
           Ok(Json(res))
        } else {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }

fn fumo() -> Router<AppState> {



    //todo!("All the reading stuff")
    Router::new().route("/list_all",get(list_all))
    
}


async fn root() -> &'static str {
    "Welcome to the fumo-API. Learn more at https://github.com/nosesisaid/fumo-api"
}





