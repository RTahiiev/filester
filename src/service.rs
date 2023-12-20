use actix_web::{get, post, web, HttpResponse, HttpRequest, Error};

use log::debug;

use crate::dir_handler::{read_folder, create_folder, BASE_DIR};
use crate::schema::Folder;
use crate::web_lib::get_body;



#[get("/")]
async fn index(req: HttpRequest) -> HttpResponse {
    let route_pattern = req.match_info().as_str();
    let full_path = BASE_DIR.to_owned() + route_pattern;
    let dir_content = read_folder(&full_path).expect("No data");
    let obj = serde_json::to_string(&dir_content).expect("Error");
    HttpResponse::Ok().json(obj)
}


#[post("/")]
async fn get_folder(payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let body = get_body(payload).await.unwrap();

    // body is loaded, now we can deserialize serde-json
    let data = serde_json::from_slice::<Folder>(&body)?;

    let route_pattern = &data.path;
    let full_path = BASE_DIR.to_owned() + route_pattern;
    let dir_content = read_folder(&full_path).expect("No data");
    let obj = serde_json::to_string(&dir_content).expect("Error");
    Ok(HttpResponse::Ok().json(obj))
}


#[post("/create")]
async fn create_folder_api(payload: web::Payload) -> Result<HttpResponse, Error> {
    debug!("create_folder");
    // payload is a stream of Bytes objects
    let body = get_body(payload).await.unwrap();
    // body is loaded, now we can deserialize serde-json
    let data = serde_json::from_slice::<Folder>(&body)?;

    let route_pattern = &data.path;
    let full_path = BASE_DIR.to_owned() + route_pattern;
    create_folder(&full_path);
    let dir_content = read_folder(&full_path).expect("No data");
    let obj = serde_json::to_string(&dir_content).expect("Error");
    Ok(HttpResponse::Ok().json(obj))
}
