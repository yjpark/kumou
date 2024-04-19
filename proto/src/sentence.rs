use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Sentence {
    pub section_id: Option<String>,
    pub text: String,
    pub clauses: Vec<Clause>,
}
