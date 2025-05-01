use crate::common::*;

use crate::service::kafka_service::*;
use crate::service::smtp_service::*;

use crate::configuration::env_config::*;

use crate::utils_module::io_utils::*;

use crate::models::topic_model::*;
use crate::models::send_id_form::*;
use crate::models::send_pw_form::*;

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
                    match self.handle_message(consumer, &msg).await {
                        Ok(_) => (),
                        Err(e) => {
                            error!("[Error][MainController->main_task] {:?}", e);
                            continue;
                        }
                    }
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
            "dev-yummy-user-pw-hist" => {
                self.user_pw_hist(consumer, message).await?;
            },
            "dev-yummy-user-id-hist" => {
                self.user_id_hist(consumer, message).await?;
            },
            _ => {
                error!("[Error][MainController] Unknown topic: {}", message.topic());
                return Err(anyhow!("Unknown topic: {}", message.topic()));
            }
        }
        
        Ok(())
    }
    #[doc = "아이디를 잊어버린 유저의 이메일로 아이디를 보내준다."]
    /// # Arguments
    /// * `message` - Kafka 메시지
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    async fn user_id_hist<'a>(&self, consumer: &StreamConsumer<DefaultConsumerContext>, message: &'a BorrowedMessage<'a>) -> Result<(), anyhow::Error> {
        
        let payload: String = self.kafka_service.get_payload_view(message)?;

        let send_id_form: SendIdForm = serde_json::from_str(payload.as_str())
            .map_err(|e| anyhow!("[Error][MainController->user_id_hist] JSON deserialization failed: {:?}", e))?;

        /* html 템플릿 */
        let html_template: String = fs::read_to_string(Path::new(HTML_ID_TEMPLATE_PATH.as_str()))?;
        let html_content: String = html_template
            .replace("{user_id}", send_id_form.user_id())
            .replace("{date}", send_id_form.now());

        self.smtp_service.send_message_to_receiver_html(send_id_form.user_email(), "아이디 찾기 결과 입니다.", &html_content).await?;

        consumer.commit_message(message, CommitMode::Async)?; /* consuming commit */
        Ok(())
    }

    #[doc = "비밀번호를 잊어버린 유저에게 임시 비밀번호를 보내준다."]
    /// # Arguments
    /// * `message` - Kafka 메시지
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    async fn user_pw_hist<'a>(&self, consumer: &StreamConsumer<DefaultConsumerContext>, message: &'a BorrowedMessage<'a>) -> Result<(), anyhow::Error> {

        let payload: String = self.kafka_service.get_payload_view(message)?;

        let send_pw_form: SendPwForm = serde_json::from_str(payload.as_str())
            .map_err(|e| anyhow!("[Error][MainController->user_id_hist] JSON deserialization failed: {:?}", e))?;

        /* html 템플릿 */
        let html_template: String = fs::read_to_string(Path::new(HTML_PW_TEMPLATE_PATH.as_str()))?;
        let html_content: String = html_template
            .replace("{user_id}", send_pw_form.user_id())
            .replace("{user_pw}", send_pw_form.user_temp_pw())
            .replace("{date}", send_pw_form.now());

        self.smtp_service.send_message_to_receiver_html(send_pw_form.user_email(), "비밀번호 찾기 결과 입니다.", &html_content).await?;
        
        consumer.commit_message(message, CommitMode::Async)?; /* consuming commit */
        Ok(())
    }
}
