use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ResponseWrapper<T> {
    #[serde(default)] // 기본값 사용
    status: Option<i8>,
    code: Option<String>,
    data: Option<T>,
}

impl<T> ResponseWrapper<T> {
    pub fn new(status: Option<i8>, code: Option<String>, data: Option<T>) -> Self {
        Self {
            status,
            code,
            data,
        }
    }

    pub fn success(data: Option<T>) -> Self {
        Self {
            status: Some(0),
            code: Some("".to_string()),
            data,
        }
    }

    pub fn fail(code: Option<String>, data: Option<T>) -> Self {
        Self {
            status: Some(-1),
            code,
            data,
        }
    }
}