use crate::common::*;

use crate::service::kafka_service::*;
use crate::service::smtp_service::*;

use crate::configuration::env_config::*;

//use crate::configuration::kafka_configuration::*;

pub struct MainController<K: KafkaService, S: SmtpService> {
    kafka_service: K,
    smtp_service: S,
}

impl<K: KafkaService, S: SmtpService> MainController<K, S> {
    pub fn new(kafka_service: K, smtp_service: S) -> Self {
        MainController {
            kafka_service,
            smtp_service,
        }
    }

    #[doc = "메인 테스크"]
    /// # Arguments
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    pub async fn main_task(&self) -> Result<(), anyhow::Error> {
        
        // let kafka_topics: TopicModelList = read_toml_from_file::<TopicModelList>("./datas/topics.toml")?;        
        let html_template: String = fs::read_to_string(Path::new(HTML_TEMPLATE_PATH.as_str()))?;
        
        self.smtp_service
            .send_message_to_receiver_html("ssh9308@mediawill.com", "test", &html_template)
            .await?;

        // let mut stream: <Result<_, anyhow::Error> as Try>::Output = self.kafka_service.get_stream_consumer()?;

        // while let Some(message) = stream.next().await {
        //     match message {
        //         Ok(msg) => {
        //             self.handle_message(&msg).await?;
        //         },
        //         Err(e) => {
        //             error!("[Error][MainController] Failed to receive message: {}", e);
        //             continue;
        //         }
        //     }
        // }

        Ok(())
    }

    //#[doc = "각 메시지를 핸들링 해주는 함수"]
    // / # Arguments
    // / * `message` - Kafka 메시지
    // /
    // / # Returns
    // / * Result<(), anyhow::Error>
    // async fn handle_message(&self, message: &Message) -> Result<(), anyhow::Error> {

    //     match message.topic() {
    //         "yummy-user-pw-hist" => {

    //         },
    //         "yummy-user-id-hist" => {

    //         },
    //         _ => {
    //             error!("[Error][MainController] Unknown topic: {}", message.topic());
    //             return Err(anyhow!("Unknown topic: {}", message.topic()));
    //         }
    //     }

    //     Ok(())
    // }

    //#[doc = ""]
    // / # Arguments
    // / * `message` - Kafka 메시지
    // /
    // / # Returns
    // / * Result<(), anyhow::Error>
    // async fn user_id_hist(&self, message: &Message) -> Result<(), anyhow::Error> {

    //     message.consumer().commit_message(&message, CommitMode::Async).; /* consuming commit */
    //     Ok(())
    // }

    //#[doc = ""]
    // / # Arguments
    // / * `message` - Kafka 메시지
    // /
    // / # Returns
    // / * Result<(), anyhow::Error>
    // async fn user_pw_hist(&self, message: &Message) -> Result<(), anyhow::Error> {

    //     message.consumer().commit_message(&message, CommitMode::Async)?; /* consuming commit */
    //     Ok(())
    // }
}
