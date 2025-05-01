use crate::common::*;

#[derive(Debug, Deserialize, Serialize, Getters, Clone)]
#[getset(get = "pub")]
pub struct TopicModel {
    pub topic_name: String,
    pub group_id: String
}

#[derive(Debug, Deserialize, Serialize, Getters, Clone)]
#[getset(get = "pub")]
pub struct TopicModelList {
    pub topic: Vec<TopicModel>,
}
