mod webhook;
mod config;
mod octocrab;

use std::env;

use actix_cors::Cors;
use actix_web::{App, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
   // init tracing_subscriber and octocrab
   tracing_subscriber::fmt::init();
   octocrab::init().unwrap();

   let args: Vec<String> = env::args().collect();
   let host = "0.0.0.0";
   let port: u16;
   // default 8080
   if args.len() == 1 {
       port = 8080;
   } else if args.len() > 1 {
       port = (&args[1]).parse::<u16>().unwrap();
   } else {
       panic!("arguments error!");
   }

   println!("App running at http://{}:{}", host, port);

   HttpServer::new(move || {
       App::new()
       .service(webhook::update_star)
       // actix-web cors config
       .wrap(Cors::default()
            .allow_any_header()
            .allow_any_origin()
            .allow_any_method()
       )  
   })
       .bind((host, port))?
       .run()
       .await
}