//use actix_cors::Cors;
//use actix_web::{http};

//pub fn new(origin: &str) -> Cors { 
//    Cors::new()
//        .allowed_origin(origin)
//        .allowed_methods(vec!["GET", "POST"])
//        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
//        .allowed_header(http::header::CONTENT_TYPE)
//        .max_age(3600)
//        .finish()
//}

/// 創建默認cors
#[macro_export]
macro_rules! cors { 
    ($hosts: expr) => { 
        actix_cors::Cors::new()
        .allowed_origin($hosts)
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::ACCEPT])
        .allowed_header(actix_web::http::header::CONTENT_TYPE)
        .max_age(3600)
        .finish()
    }
}
