use actix_web::{App, HttpServer, middleware, web};

mod db;
mod services;
mod tests;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://0.0.0.0:8000");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(services::data_processing::index))
                .service(web::resource("/hello/{name}").to(services::data_processing::greet))
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}
