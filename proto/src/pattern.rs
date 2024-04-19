use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Pattern {
    Prefix(Word),
    Suffix(Word),
    Infix(Word),
    Complex(Vec<PatternPiece>),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternClass {
    Unknown,
    Noun,
    Verb,
    Adjective,
    Adverb,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternPiece {
    Word(Word),
    Text,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Match {
    pub text: String,
    pub pattern: Pattern,
    pub pieces: Vec<Piece>,
}

impl Pattern {
    pub fn to_pieces(&self) -> Vec<PatternPiece> {
        match self {
            Pattern::Prefix(word) => vec![
                PatternPiece::Word(word.clone()),
                PatternPiece::Text,
            ],
            Pattern::Suffix(word) => vec![
                PatternPiece::Text,
                PatternPiece::Word(word.clone()),
            ],
            Pattern::Infix(word) => vec![
                PatternPiece::Text,
                PatternPiece::Word(word.clone()),
                PatternPiece::Text,
            ],
            Pattern::Complex(pieces) => pieces.clone(),
        }
    }
}