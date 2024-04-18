use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Section<'a> {
    pub id: Option<&'a str>,
    pub title: Option<&'a str>,
    pub parent_id: Option<&'a str>,
}
