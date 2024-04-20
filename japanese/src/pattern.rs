use lazy_static::lazy_static;
use crate::prelude::*;

lazy_static! {
    pub static ref WA: Pattern = Pattern::new(
        PatternPieces::Suffix(particle::WA.clone()),
    );
    pub static ref GA: Pattern = Pattern::new(
        PatternPieces::Suffix(particle::GA.clone()),
    );
    pub static ref O: Pattern = Pattern::new(
        PatternPieces::Suffix(particle::O.clone()),
    );
    pub static ref TO: Pattern = Pattern::new(
        PatternPieces::Suffix(particle::TO.clone()),
    );
    pub static ref DE: Pattern = Pattern::new(
        PatternPieces::Suffix(particle::DE.clone()),
    );
    pub static ref NI: Pattern = Pattern::new(
        PatternPieces::Suffix(particle::NI.clone()),
    );
    pub static ref E: Pattern = Pattern::new(
        PatternPieces::Suffix(particle::E.clone()),
    );
    pub static ref NO: Pattern = Pattern::new(
        PatternPieces::Suffix(particle::NO.clone()),
    );
    pub static ref MO: Pattern = Pattern::new(
        PatternPieces::Suffix(particle::MO.clone()),
    );
    pub static ref NE: Pattern = Pattern::new(
        PatternPieces::Suffix(particle::NE.clone()),
    );
    pub static ref NA: Pattern = Pattern::new(
        PatternPieces::Suffix(particle::NA.clone()),
    );
}