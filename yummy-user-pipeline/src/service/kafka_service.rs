use crate::common::*;

use crate::configuration::env_config::*;

use crate::configuration::kafka_configuration::*;

use crate::utils_module::io_utils::*;

use crate::models::topic_model::*;

#[async_trait]
pub trait KafkaService {
    fn get_stream_consumer(
        &self,
    ) -> Result<MessageStream<'static, DefaultConsumerContext>, anyhow::Error>;
    fn get_consumer(&self) -> &'static StreamConsumer<DefaultConsumerContext>;
    fn get_payload_view<'a>(
        &self,
        message: &'a BorrowedMessage<'a>,
    ) -> Result<String, anyhow::Error>;
    fn get_stream_consumer_for(
        &self,
        topic: &str,
        group_id: &str,
    ) -> Result<StreamConsumer, anyhow::Error>;
}

#[derive(Debug, new)]
pub struct KafkaServicePub {}

#[async_trait]
impl KafkaService for KafkaServicePub {
    fn get_consumer(&self) -> &'static StreamConsumer<DefaultConsumerContext> {
        &KAFKA_CONSUMER
    }

    #[doc = "카프카 컨슈머 객체를 생성하여 리턴. - 병렬처리를 고려하지 않은 함수"]
    /// # Arguments
    ///
    /// # Returns
    /// * Result<MessageStream<'static, DefaultConsumerContext>, anyhow::Error>
    fn get_stream_consumer(
        &self,
    ) -> Result<MessageStream<'static, DefaultConsumerContext>, anyhow::Error> {
        let kafka_topics: TopicModelList = read_toml_from_file::<TopicModelList>(&TOPICS_PATH)?;
        let topic_list: Vec<&str> = kafka_topics
            .topic()
            .iter()
            .map(|topic| topic.topic_name().as_str())
            .collect::<Vec<_>>();

        KAFKA_CONSUMER
            .subscribe(&topic_list)
            .expect("[Error][KafkaService] Failed to subscribe to topics.");

        let stream: rdkafka::consumer::MessageStream<
            '_,
            rdkafka::consumer::DefaultConsumerContext,
        > = KAFKA_CONSUMER.stream();

        Ok(stream)
    }

    #[doc = "컨슈밍한 토픽의 데이터를 뽑아주는 함수"]
    /// # Arguments
    ///
    /// # Returns
    /// * Result<String, anyhow::Error>
    fn get_payload_view<'a>(
        &self,
        message: &'a BorrowedMessage<'a>,
    ) -> Result<String, anyhow::Error> {
        let payload: &str = match message.payload_view::<str>() {
            Some(Ok(json_str)) => json_str,
            Some(Err(e)) => {
                return Err(anyhow::anyhow!(
                    "[Error][KafkaService->get_patload_view] Payload UTF-8 decoding error: {:?}",
                    e
                ));
            }
            None => {
                return Err(anyhow::anyhow!(
                    "[Error][KafkaService->get_patload_view] Empty payload"
                ));
            }
        };

        Ok(payload.to_string())
    }

    #[doc = "멀티스레드 실행을 위해 StreamConsumer 을 싱글톤으로 가져오지 않는다. - 벙렬처리를 고려한 함수"]
    /// # Arguments
    /// * `topic` - 토픽이름
    /// * `group_id` - 그룹 아이디
    /// 
    /// # Returns
    /// * Result<StreamConsumer, anyhow::Error>
    fn get_stream_consumer_for(
        &self,
        topic: &str,
        group_id: &str,
    ) -> Result<StreamConsumer, anyhow::Error> {
        let hosts: String =
            env::var("KAFKA_HOST").expect("[ENV file read Error] 'KAFKA_HOST' must be set.");
        let sasl_user_name: String = env::var("KAFKA_SASL_USERNAME")
            .expect("[ENV file read Error] 'KAFKA_SASL_USERNAME' must be set.");
        let sasl_passwd: String = env::var("KAFKA_SASL_PASSWORD")
            .expect("[ENV file read Error] 'KAFKA_SASL_PASSWORD' must be set.");

        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &hosts)
            .set("group.id", group_id)
            .set("security.protocol", "SASL_PLAINTEXT")
            .set("sasl.mechanisms", "PLAIN")
            .set("sasl.username", &sasl_user_name)
            .set("sasl.password", &sasl_passwd)
            .set("auto.offset.reset", "earliest")
            .set("enable.auto.commit", "false")
            .create()
            .map_err(|e| anyhow!("[KafkaService] Failed to create consumer: {:?}", e))?;

        consumer.subscribe(&[topic]).map_err(|e| {
            anyhow!(
                "[KafkaService] Failed to subscribe to topic: {}: {:?}",
                topic,
                e
            )
        })?;

        Ok(consumer)
    }
}
