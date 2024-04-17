use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Sentence {
    pub uuid: Uuid,
    pub section: Uuid,
    pub content: String,
}
