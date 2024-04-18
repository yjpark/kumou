use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Section {
    pub id: Option<String>,
    pub title: Option<String>,
    pub parent_id: Option<String>,
}
