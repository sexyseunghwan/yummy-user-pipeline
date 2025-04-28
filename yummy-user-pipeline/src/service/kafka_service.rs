use crate::common::*;

// use crate::configuration::kafka_configuration::*;

// use crate::utils_module::io_utils::*;

// use crate::models::topic_model::*;

#[async_trait]
pub trait KafkaService {
    //fn get_stream_consumer(&self) -> Result<MessageStream<'static, DefaultConsumerContext>, anyhow::Error>;
}

#[derive(Debug, new)]
pub struct KafkaServicePub {}

#[async_trait]
impl KafkaService for KafkaServicePub {
    // #[doc = "카프카 컨슈머 객체를 생성하여 리턴."]
    // /// # Arguments
    // ///
    // /// # Returns
    // /// * Result<MessageStream<'static, DefaultConsumerContext>, anyhow::Error>
    // fn get_stream_consumer(&self) -> Result<MessageStream<'static, DefaultConsumerContext>, anyhow::Error> {

    //     let kafka_topics: TopicModelList = read_toml_from_file::<TopicModelList>("./datas/topics.toml")?;
    //     let topic_list: Vec<&str> = kafka_topics.topic_list().iter().map(|topic| topic.topic_name()).collect::<Vec<_>>();

    //     KAFKA_CONSUMER
    //         .subscribe(&topic_list)
    //         .expect("[Error][KafkaService] Failed to subscribe to topics.");

    //     let stream: rdkafka::consumer::MessageStream<'_, rdkafka::consumer::DefaultConsumerContext> = KAFKA_CONSUMER.stream();
    //     Ok(stream)

    // }
}
