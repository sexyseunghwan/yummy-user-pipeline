use crate::common::*;

#[doc = "Function to globally initialize the 'HTML_TEMPLATE_PATH' variable"]
pub static HTML_TEMPLATE_PATH: once_lazy<String> = once_lazy::new(|| {
    dotenv().ok();
    env::var("HTML_TEMPLATE_PATH").expect("[ENV file read Error] 'HTML_TEMPLATE_PATH' must be set")
});
