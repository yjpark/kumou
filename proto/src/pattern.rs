use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pattern {
    pub id: String,
    pub pieces: PatternPieces,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternPieces {
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

impl PatternPiece {
    pub fn id(&self) -> &str {
        match self {
            PatternPiece::Word(word) => word.id(),
            PatternPiece::Text => Pattern::WILDCAST_ID,
        }
    }
}

impl Pattern {
    pub const WILDCAST_ID: &'static str = "*";
    pub const SEPARATOR_ID: &'static str = " ";

    pub fn new(pieces: PatternPieces) -> Self {
        let id = pieces.id();
        Self {
            id,
            pieces,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

impl PatternPieces {
    pub fn to_pieces(&self) -> Vec<PatternPiece> {
        match self {
            PatternPieces::Prefix(word) => vec![
                PatternPiece::Word(word.clone()),
                PatternPiece::Text,
            ],
            PatternPieces::Suffix(word) => vec![
                PatternPiece::Text,
                PatternPiece::Word(word.clone()),
            ],
            PatternPieces::Infix(word) => vec![
                PatternPiece::Text,
                PatternPiece::Word(word.clone()),
                PatternPiece::Text,
            ],
            PatternPieces::Complex(pieces) => pieces.clone(),
        }
    }

    pub fn id(&self) -> String {
        self.to_pieces()
            .into_iter()
            .map(|x| x.id().to_owned())
            .collect::<Vec<String>>()
            .join(Pattern::SEPARATOR_ID)
    }
}