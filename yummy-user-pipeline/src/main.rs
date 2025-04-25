mod common;
use common::*;

mod utils_module;
use utils_module::logger_utils::*;

mod models;

mod service;

mod controller;

mod configuration;

#[tokio::main]
async fn main() {
    
    dotenv().ok();
    info!("Program start!");

    


}
