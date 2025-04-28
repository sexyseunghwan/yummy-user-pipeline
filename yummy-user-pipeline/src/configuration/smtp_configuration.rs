use crate::common::*;

use crate::models::smtp_config::*;

#[doc = "SMTP 연결 싱글톤 객체"]
pub static SMTP_TRANSPORT: once_lazy<Arc<AsyncSmtpTransport<Tokio1Executor>>> =
    once_lazy::new(|| {
        // let smtp_name: String = env::var("SMTP_NAME").expect("[ENV file read Error] 'SMTP_NAME' must be set.");
        // let credential_id: String = env::var("CREDENTIAL_ID").expect("[ENV file read Error] 'CREDENTIAL_ID' must be set.");
        // let credential_pw: String = env::var("CREDENTIAL_PW").expect("[ENV file read Error] 'CREDENTIAL_PW' must be set.");

        let smtp_config: &SmtpConfig = get_smtp_config_info();

        let creds: Credentials = Credentials::new(
            smtp_config.credential_id().to_string(),
            smtp_config.credential_pw().to_string(),
        );

        // let mailer: AsyncSmtpTransport<Tokio1Executor> = AsyncSmtpTransport::<Tokio1Executor>::relay(
        //     smtp_name.as_str(),
        // )
        // .expect("[Error][SMTP_TRANSPORT] SMTP server connection failed.")
        // .credentials(creds)
        // .build();

        // Arc::new(mailer)

        Arc::new(
            AsyncSmtpTransport::<Tokio1Executor>::relay(smtp_config.smtp_name())
                .expect("[Error][SMTP_TRANSPORT] SMTP server connection failed.")
                .credentials(creds)
                .build(),
        )
    });
