use inquire::InquireError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug,Error)]
// 自定义Error
pub enum Error{
    #[error(" -> io error : {0}")]
    IoError(#[from] std::io::Error),
    #[error(" -> request error : {0}")]
    RequestError(#[from] reqwest::Error),
    #[error(" -> error : {0}")]
    BizError(&'static str),
    #[error(" -> inquire error : {0}")]
    InquireError(#[from] InquireError),
}

