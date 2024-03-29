use actix_web::{error, web, Error};
use futures::StreamExt;


const MAX_SIZE: usize = 262_144; // max payload size is 256k


pub async fn get_body(mut payload: web::Payload) -> Result<web::BytesMut, Error>{
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}
