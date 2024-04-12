//! 

use redis::{Client, Commands};
use std::sync::RwLock;
use lazy_static::lazy_static;
use uuid::Uuid;

use crate::SETTINGS;

use axum::extract::Json;
use serde::{Deserialize, Serialize};

// Global REDIS
lazy_static! {
    static ref REDIS: RwLock<Client> = RwLock::new(
        redis::Client::open(format!(
            "redis://{}:{}",
            SETTINGS.read().unwrap().get::<String>("REDIS_IP").unwrap(),
            SETTINGS.read().unwrap().get::<String>("REDIS_PORT").unwrap(),
        ))
        .unwrap()
    );
}


// #[derive(Deserialize, Serialize)]
// pub struct Job {
//     id: String,
//     trigger: String,
//     commit: String
// }

/// Enqueue a json object to Redis for later use
// pub async fn enqueue(Json(payload): Json<Job>){
pub async fn enqueue(payload: String){
    
    // Get the redis connection
    let mut conn = REDIS.read().unwrap().get_connection().unwrap();
    // conn.set(Uuid::new_v4().to_string(), serde_json::to_string(&payload).unwrap()).unwrap();
    let _: () = conn.set(Uuid::new_v4().to_string(), &payload).unwrap();
}

