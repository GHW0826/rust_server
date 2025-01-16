use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct Tests {
    #[validate(range(min = 0, max = 9999))]
    pub id: Option<i64>
}

impl Tests {
    pub fn new(id: Option<i64>) -> Self {
        Self {
            id,
        }
    }
}