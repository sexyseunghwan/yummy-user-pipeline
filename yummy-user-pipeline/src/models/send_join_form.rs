use crate::common::*;

#[derive(Debug, Deserialize, Serialize, Getters, Clone)]
#[getset(get = "pub")]
pub struct SendJoinForm {
    pub now: String,
    #[serde(rename = "userEmail")]
    pub user_email: String,
    #[serde(rename = "joinCheckCode")]
    pub join_check_code: String
}
