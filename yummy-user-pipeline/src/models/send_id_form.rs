use crate::common::*;

#[derive(Debug, Deserialize, Serialize, Getters, Clone)]
#[getset(get = "pub")]
pub struct SendIdForm {
    pub now: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userEmail")]
    pub user_email: String,
}
