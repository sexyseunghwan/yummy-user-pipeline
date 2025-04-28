use crate::common::*;

use crate::service::kafka_service::*;
use crate::service::smtp_service::*;

use crate::configuration::env_config::*;

use crate::utils_module::io_utils::*;

use crate::models::topic_model::*;

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
        
        //let html_template: String = fs::read_to_string(Path::new(HTML_TEMPLATE_PATH.as_str()))?;

        // self.smtp_service
        //     .send_message_to_receiver_html("ssh9308@mediawill.com", "test", &html_template)
        //     .await?;

        let kafka_topics: TopicModelList = read_toml_from_file::<TopicModelList>(&TOPICS_PATH)?;
        let consumer: &StreamConsumer<DefaultConsumerContext> = self.kafka_service.get_consumer();
        let mut stream: MessageStream<'_, DefaultConsumerContext> = self.kafka_service.get_stream_consumer()?;

        while let Some(message) = stream.next().await {
            match message {
                Ok(msg) => {
                    self.handle_message(consumer, &msg).await?;
                },
                Err(e) => {
                    error!("[Error][MainController] Failed to receive message: {}", e);
                    continue;
                }
            }
        }

        Ok(())
    }

    #[doc = "각 메시지를 핸들링 해주는 함수"]
    /// # Arguments
    /// * `message` - Kafka 메시지
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    async fn handle_message<'a>(&self, consumer: &StreamConsumer<DefaultConsumerContext>, message: &'a BorrowedMessage<'a>) -> Result<(), anyhow::Error> {

        match message.topic() {
            "yummy-user-pw-hist" => {
                
            },
            "yummy-user-id-hist" => {

            },
            _ => {
                error!("[Error][MainController] Unknown topic: {}", message.topic());
                return Err(anyhow!("Unknown topic: {}", message.topic()));
            }
        }

        Ok(())
    }
    #[doc = ""]
    /// # Arguments
    /// * `message` - Kafka 메시지
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    async fn user_id_hist<'a>(&self, consumer: &StreamConsumer<DefaultConsumerContext>, message: &'a BorrowedMessage<'a>) -> Result<(), anyhow::Error> {

        consumer.commit_message(message, CommitMode::Async)?; 
        //message.commit_message(&message, CommitMode::Async)?; /* consuming commit */
        Ok(())
    }

    #[doc = ""]
    /// # Arguments
    /// * `message` - Kafka 메시지
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    async fn user_pw_hist<'a>(&self, consumer: &StreamConsumer<DefaultConsumerContext>, message: &'a BorrowedMessage<'a>) -> Result<(), anyhow::Error> {

        consumer.commit_message(message, CommitMode::Async)?; /* consuming commit */
        Ok(())
    }
}
