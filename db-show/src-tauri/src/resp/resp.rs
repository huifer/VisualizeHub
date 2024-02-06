use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub status_code: i32,
    pub description: String,
    pub data: Option<T>,
}

pub static SUCCESS: i32 = 20000;

impl<T> Response<T> {
    pub fn new(description: impl Into<String>, data: Option<T>) -> Self {
        Self {
            status_code: SUCCESS,
            description: description.into(),
            data,
        }
    }
    pub fn ok() -> Self {
        Self {
            status_code: SUCCESS,
            description: "操作成功".to_string(),
            data: None,
        }
    }
    pub fn from_error(description: impl Into<String>) -> Self {
        Self {
            status_code: -1,
            description: description.into(),
            data: None,
        }
    }
}
