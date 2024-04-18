pub mod word_class;
pub mod word;
pub mod clause;
pub mod piece;
pub mod pattern;

pub mod prelude {
    pub use crate::word_class::WordClass;
    pub use crate::word::Word;
    pub use crate::clause::Clause;
    pub use crate::piece::Piece;
    pub use crate::pattern::{Pattern, PatternPiece, Match};
}