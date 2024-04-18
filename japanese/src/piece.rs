use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Piece {
    Text(String),
    Word(Word),
    Match(Match),
    Clause(Clause),
}
