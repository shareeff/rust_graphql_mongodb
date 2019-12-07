#[macro_use]
extern crate juniper;

#[macro_use]
extern crate serde;


use std::io;
use std::sync::Arc;

use actix_web::{web, App, Error, HttpResponse, HttpServer};
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use lazy_static::lazy_static;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::Collection;

mod graphql_schema;
mod models;
mod services;

use crate::graphql_schema::{create_schema, Schema};



fn main() -> io::Result<()> {
    let schema = std::sync::Arc::new(create_schema());
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            
    })
    .bind("localhost:8080")?
    .run()
}

lazy_static! {
    pub static ref MONGO: Client = create_mongo_client();
}

fn create_mongo_client() -> Client {
    Client::connect("localhost", 27017)
        .expect("Failed to initialize standalone client.")
}

fn collection(coll_name: &str) -> Collection {
    MONGO.db("myteam").collection(coll_name)
}

fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}

fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://localhost:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

