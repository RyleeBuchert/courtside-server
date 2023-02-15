// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
// use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
// use std::env;
// use std::error::Error;
// use tokio;

// struct AppState {
//     client: Client,
// }

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[get("/team")]
// async fn get_team(data: web::Data<AppState>) -> impl Responder {
//     // Get a handle to the collection "teams":
//     let collection = &data.client.database("courtside").collection("teams");

// }

// // A function to connect to the mongodb
// async fn connect_to_mongodb() -> Result<Client, Box<dyn Error>> {
//     // Connect to mongodb
//     let client_uri =
//         env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

//     // A Client is needed to connect to MongoDB:
//     let options =
//         ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
//             .await?;
//     let client = Client::with_options(options)?;

//     // Get a handle to the database "courtside":
//     let db = client.database("courtside");

//     Ok(client)
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let client = connect_to_mongodb().await.unwrap();
//     HttpServer::new(move || {
//         App::new()
//             .app_data(AppState {
//                 client: client.clone(),
//             })
//             .service(hello)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }


/*
A web server which connects to the mongodb database and returns the teams in the database
 */

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use tokio;

struct AppState {
    client: Client,
}

#[get("/team")]
async fn get_team(data: web::Data<AppState>) -> impl Responder {
    // Get a handle to the collection "teams":
    let collection = &data.client.database("courtside").collection("teams");

    let team: Document = collection
        .find_one(doc! {"name": "Lakers"}, None)
        .await
        .unwrap()
        .unwrap();
}

// A function to connect to the mongodb
async fn connect_to_mongodb() -> Result<Client, Box<dyn Error>> {
    // Connect to mongodb
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    Ok(client)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = connect_to_mongodb().await.unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(AppState {
                client: client.clone(),
            })
            .service(get_team)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}