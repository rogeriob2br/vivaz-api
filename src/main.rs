mod configs;
use actix_web::{web, App, HttpResponse, HttpServer};
use crate::configs::reader_cfg::{RedisConfig, SettingsReader};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SETTINGS: SettingsReader = SettingsReader::new("Settings.toml", "");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_config = &SETTINGS.redis;

    HttpServer::new(move || {
        App::new().data(redis_config).service(
            web::resource("/api/test")
                .route(web::get().to(get_key)),
        )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}