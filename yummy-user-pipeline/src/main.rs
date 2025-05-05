/*
Author      : Seunghwan Shin
Create date : 2025-05-05
Description : 아이디/비밀번호 찾기 위한 배치 프로그램

History     : 2025-05-05 Seunghwan Shin       # [v.1.0.0] first create
*/

mod common;
use common::*;

mod utils_module;
use utils_module::logger_utils::*;
use utils_module::check_util::*;

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

    Arc::new(main_controller)
        .run_parallel()
        .await
        .expect("Parallel task failed");
}
