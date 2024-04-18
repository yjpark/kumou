use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WordClass {
    Unknown,
    Particle,
    Noun,
    Verb,
    Adjective,
    Adverb,
}