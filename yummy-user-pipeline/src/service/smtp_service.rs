use crate::common::*;

use crate::configuration::smtp_configuration::*;

use crate::models::smtp_config::*;

#[async_trait]
pub trait SmtpService {
    async fn send_message_to_receiver_html(
        &self,
        email_id: &str,
        subject: &str,
        html_content: &str,
    ) -> Result<(), anyhow::Error>;
}

#[derive(Debug, new)]
pub struct SmtpServicePub {}

#[async_trait]
impl SmtpService for SmtpServicePub {
    #[doc = "이메일을 보내는 함수"]
    async fn send_message_to_receiver_html(
        &self,
        email_id: &str,
        subject: &str,
        html_content: &str,
    ) -> Result<(), anyhow::Error> {
        let smtp_config: &SmtpConfig = get_smtp_config_info();

        let from_addr: Mailbox = match smtp_config.credential_id().parse() {
            Ok(addr) => addr,
            Err(e) => return Err(anyhow!("[Error][SmtpService->send_message_to_receiver_html] Failed to parse credential email address: {:?}", e)),
        };

        let to_addr: Mailbox  = match email_id.parse() {
            Ok(addr) => addr,
            Err(e) => return Err(anyhow!("[Error][SmtpService->send_message_to_receiver_html] Failed to parse receiver email address: {:?}", e)),
        };

        

        let email: Message = Message::builder()
            .from(from_addr)
            .to(to_addr)
            .subject(subject)
            .multipart(
                MultiPart::alternative().singlepart(SinglePart::html(html_content.to_string())),
            )?;

        match SMTP_TRANSPORT.send(email).await {
            Ok(_) => {
                info!("[SMTP] Email sent to {} successfully.", email_id);
                Ok(())
            }
            Err(e) => Err(anyhow!("{:?} : Failed to send email to {} ", e, email_id)),
        }
    }
}
