use actix_cors::Cors;
use actix_web::{App, HttpServer};
use n_star_api::controllers;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 接收命令行参数, 绑定动态端口
    let args: Vec<String> = env::args().collect();
    let host = "0.0.0.0";
    let port: u16;
    // 如果没有传参，默认8080
    if args.len() == 1 {
        port = 8080;
    } else if args.len() > 1 {
        port = (&args[1]).parse::<u16>().unwrap();
    } else {
        panic!("命令行参数获取错误");
    }

    println!("Webhook running at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
        .service(controllers::hook)
        // actix-web解决跨域
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