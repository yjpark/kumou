use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Sentence<'a> {
    pub section_id: Option<&'a str>,
    pub text: &'a str,
}
