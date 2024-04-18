use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Sentence {
    pub section: Option<Id>,
    pub content: String,
}
