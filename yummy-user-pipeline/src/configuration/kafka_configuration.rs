use crate::common::*;

#[doc = "카프카 컨슈머 싱글톤 객체 -> 병렬사용시 부적합."]
pub static KAFKA_CONSUMER: once_lazy<StreamConsumer> = once_lazy::new(|| {
    let hosts: String =
        env::var("KAFKA_HOST").expect("[ENV file read Error] 'KAFKA_HOST' must be set.");
    let sasl_user_name: String = env::var("KAFKA_SASL_USERNAME")
        .expect("[ENV file read Error] 'KAFKA_SASL_USERNAME' must be set.");
    let sasl_passwd: String = env::var("KAFKA_SASL_PASSWORD")
        .expect("[ENV file read Error] 'KAFKA_SASL_PASSWORD' must be set.");

    ClientConfig::new()
        .set("bootstrap.servers", hosts)
        .set("group.id", "rust-consumer-group")
        .set("security.protocol", "SASL_PLAINTEXT")
        .set("sasl.mechanisms", "PLAIN")
        .set("sasl.username", sasl_user_name)
        .set("sasl.password", sasl_passwd)
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "false") /* 수동커밋 */
        .create::<StreamConsumer>()
        .expect("Failed to create Kafka consumer")
});
