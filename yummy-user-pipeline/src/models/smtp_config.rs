use crate::common::*;

#[derive(Debug, Deserialize, Serialize, Getters, Clone)]
#[getset(get = "pub")]
pub struct SmtpConfig {
    pub smtp_name: String,
    pub credential_id: String,
    pub credential_pw: String,
}

static SMTP_CONFIG: once_lazy<SmtpConfig> = once_lazy::new(|| {
    let smtp_name: String = env::var("SMTP_NAME").expect("[ENV] SMTP_NAME must be set");
    let credential_id: String = env::var("CREDENTIAL_ID").expect("[ENV] CREDENTIAL_ID must be set");
    let credential_pw: String = env::var("CREDENTIAL_PW").expect("[ENV] CREDENTIAL_PW must be set");

    SmtpConfig {
        smtp_name,
        credential_id,
        credential_pw,
    }
});

#[doc = "어디서든 호출해서 &SmtpConfig 얻어올 수 있는 헬퍼 함수"]
pub fn get_smtp_config_info() -> &'static SmtpConfig {
    &SMTP_CONFIG
}
