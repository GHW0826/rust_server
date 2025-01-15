use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Tests {
    #[serde(default)] // 기본값 사용
    id: Option<i64>
}
