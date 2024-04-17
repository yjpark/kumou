use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Section {
    pub uuid: Uuid,
    pub parent: Option<Uuid>,
    pub title: Option<String>,
}