mod dir_handler;
mod file_handler;
mod service;
mod schemas;
mod web_lib;

use actix_web::{App, HttpServer};
use dir_handler::{path_exist, create_folder, BASE_DIR};
use service::{index, create_folder_api};
use log::info;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Start app...");
    let base_path = BASE_DIR;

    if !path_exist(base_path) {
        info!("Not exist {}", base_path);
        let _ = create_folder(base_path);
    }

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(create_folder_api)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
