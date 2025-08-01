use axum::{
    Router,
    routing::{get, post},

};
use crate::handlers::ocel::{post_ocel_binary,test_post_handler,test_get_handler, upload_handler};

pub fn router() -> Router {
    Router::new()
        //.route("/", post(post_ocel_binary))
        //.route("/", get(get_ocel))
        .route("/", post(test_post_handler))
        .route("/test", post(upload_handler))
        .route("/", get(test_get_handler))
        
}