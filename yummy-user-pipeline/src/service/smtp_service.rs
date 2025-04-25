use crate::common::*;

#[async_trait]
pub trait SmtpService {
    
}

#[derive(Debug, new)]
pub struct SmtpServicePub {}


#[async_trait]
impl SmtpService for SmtpServicePub {
    
}