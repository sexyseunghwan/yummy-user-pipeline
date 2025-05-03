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
    
    let arc_smtp_service: Arc<SmtpServicePub> = Arc::new(smtp_service);
    let arc_kafka_service: Arc<KafkaServicePub> = Arc::new(kafka_service);
    

    let main_controller: MainController<Arc<KafkaServicePub>, Arc<SmtpServicePub>> =
        MainController::new(arc_kafka_service, arc_smtp_service);

    Arc::new(main_controller)
        .run_parallel()
        .await
        .expect("Parallel task failed");
}
