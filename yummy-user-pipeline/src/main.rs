mod common;
use common::*;

mod utils_module;
use utils_module::logger_utils::*;

mod models;

mod service;
use service::{kafka_service::*, smtp_service::*};

mod controller;
use controller::main_controller::*;

mod configuration;

#[tokio::main]
async fn main() {
    dotenv().ok();
    set_global_logger();
    info!("Program start!");

    let smtp_service: SmtpServicePub = SmtpServicePub::new();
    let kafka_service: KafkaServicePub = KafkaServicePub::new();

    let main_controller: MainController<KafkaServicePub, SmtpServicePub> =
        MainController::new(kafka_service, smtp_service);

    main_controller.main_task().await.unwrap();
}
