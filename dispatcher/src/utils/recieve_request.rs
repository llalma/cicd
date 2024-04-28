//!

use lazy_static::lazy_static;
use redis::{Client, Commands};
use std::sync::RwLock;
use uuid::Uuid;

use crate::SETTINGS;
use std::collections::HashMap;

// Global REDIS
lazy_static! {
    static ref REDIS: RwLock<Client> = RwLock::new(
        redis::Client::open(format!(
            "redis://{}:{}",
            SETTINGS.read().unwrap().get::<String>("REDIS_IP").unwrap(),
            SETTINGS
                .read()
                .unwrap()
                .get::<String>("REDIS_PORT")
                .unwrap(),
        ))
        .unwrap()
    );
    static ref POST_CLIENT: RwLock<reqwest::Client> = RwLock::new(reqwest::Client::new());
}

pub async fn enqueue(payload: String) {
    // Get the redis connection
    let mut conn = REDIS.read().unwrap().get_connection().unwrap();

    // Create an ID for job
    let job_id = Uuid::new_v4().to_string();

    let _: () = conn.set(&job_id, &payload).unwrap();

    // Trigger worker
    trigger_worker(job_id);
}

async fn trigger_worker(job_id: String) {
    let endpoint: String = SETTINGS
        .read()
        .unwrap()
        .get::<String>("WORKER_ENDPOINT")
        .unwrap();

    let body = HashMap::from([("id", job_id)]);

    let _ = POST_CLIENT
        .read()
        .unwrap()
        .post(endpoint)
        .json(&body)
        .send()
        .await;
}
