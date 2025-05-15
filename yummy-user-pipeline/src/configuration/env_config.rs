use crate::common::*;

#[doc = "Function to globally initialize the 'HTML_ID_TEMPLATE_PATH' variable"]
pub static HTML_ID_TEMPLATE_PATH: once_lazy<String> = once_lazy::new(|| {
    dotenv().ok();
    env::var("HTML_ID_TEMPLATE_PATH")
        .expect("[ENV file read Error] 'HTML_ID_TEMPLATE_PATH' must be set")
});

#[doc = "Function to globally initialize the 'HTML_PW_TEMPLATE_PATH' variable"]
pub static HTML_PW_TEMPLATE_PATH: once_lazy<String> = once_lazy::new(|| {
    dotenv().ok();
    env::var("HTML_PW_TEMPLATE_PATH")
        .expect("[ENV file read Error] 'HTML_PW_TEMPLATE_PATH' must be set")
});

#[doc = "Function to globally initialize the 'HTML_JOIN_CHECK_TEMPLATE_PATH' variable"]
pub static HTML_JOIN_CHECK_TEMPLATE_PATH: once_lazy<String> = once_lazy::new(|| {
    dotenv().ok();
    env::var("HTML_JOIN_CHECK_TEMPLATE_PATH")
        .expect("[ENV file read Error] 'HTML_JOIN_CHECK_TEMPLATE_PATH' must be set")
});

#[doc = "Function to globally initialize the 'TOPICS_PATH' variable"]
pub static TOPICS_PATH: once_lazy<String> = once_lazy::new(|| {
    dotenv().ok();
    env::var("TOPICS_PATH").expect("[ENV file read Error] 'TOPICS_PATH' must be set")
});
