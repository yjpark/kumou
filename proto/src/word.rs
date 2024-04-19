use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Word {
    pub text: String,
}

impl Word {
    pub fn id(&self) -> &str {
        &self.text
    }
}