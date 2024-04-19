pub use unic_langid;

pub mod word;
pub mod clause;
pub mod piece;
pub mod pattern;
pub mod sentence;
pub mod section;

pub mod prelude {
    pub use unic_langid::LanguageIdentifier;

    pub use crate::sentence::Sentence;
    pub use crate::section::Section;
    pub use crate::word::Word;
    pub use crate::clause::Clause;
    pub use crate::piece::Piece;
    pub use crate::pattern::{Pattern, PatternPiece, Match};
}