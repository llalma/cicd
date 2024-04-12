use config::Config;
use lazy_static::lazy_static;
use std::sync::RwLock;

// Global Configs
lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(
            Config::builder()
            .add_source(config::File::with_name("Config.toml"))
            .build()
            .unwrap()
    );
}

extern crate axum;
use axum::{
    routing::post,
    Router,
};

mod utils;
use utils::recieve_request;

#[tokio::main]
async fn main() {
    // Read config file
    // let configs = Config::builder()
    //     .add_source(config::File::with_name("Config.toml"))
    //     .build()
    //     .unwrap();

    // SETTINGS.write()


    // println!("{}", configs.get::<String>("HOST_IP").unwrap()); 
    // build our application with a single route
    let app = Router::new().route("/enqueue", post(recieve_request::enqueue));

   // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!(
            "{}:{}",
            SETTINGS.read().unwrap().get::<String>("HOST_IP").unwrap(),
            SETTINGS.read().unwrap().get::<String>("HOST_PORT").unwrap()))
        .await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

