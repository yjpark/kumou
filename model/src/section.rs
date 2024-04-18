use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Section {
    pub uuid: Option<Id>,
    pub title: Option<String>,
    pub parent: Option<Id>,
    // runtime fields
    pub childrens: Vec<Section>,
    pub sentences: Vec<Sentence>,
}