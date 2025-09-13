//----Trying to define multiple routes
// use axum::{routing::get,Router};
// #[tokio::main]
// async fn main(){
//         let app=Router::new()
//                 .route("/",get(|| async {"first page"}))
//                 .route("/second",get(|| async{"second page"}))
//                 .route("/third",get(|| async {"third page"}));
//         let listener=tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//         let _=axum::serve(listener,app).await.unwrap();
        
// }
//---Extractors
use axum::{
    extract::{Request, Json, Path, Extension, Query},
    routing::{post,get},
    http::header::HeaderMap,
    body::{Bytes, Body},
    Router,
};
use serde_json::Value;
use std::collections::HashMap;
#[tokio::main]
async fn main(){
        let app=Router::new()
                .route("/path/{user_id}",get(path))
                .route("/query",get(query))
                .route("/json",axum::routing::post(json));
        let listener=tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        let _=axum::serve(listener,app).await.unwrap();
}
async fn path(Path(user_id):Path<u32>)->String{
        format!("the user id is : {:?}",user_id)
}
async fn query(Query(params):Query<HashMap<String,String>>)->String{
        format!("the param is: {:?}",params)
}
async fn json(Json(load):Json<Value>)->String{
        format!("the load is: {:?}",load)
}
