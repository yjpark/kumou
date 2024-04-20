use lazy_static::lazy_static;
use crate::prelude::*;

lazy_static! {
    pub static ref WA: Word = Word::new("は");

    pub static ref GA: Word = Word::new("が");
    pub static ref O: Word = Word::new("を");

    pub static ref TO: Word = Word::new("と");

    pub static ref DE: Word = Word::new("で");
    pub static ref NI: Word = Word::new("に");
    pub static ref E: Word = Word::new("へ");

    pub static ref NO: Word = Word::new("の");
    pub static ref MO: Word = Word::new("も");

    pub static ref NE: Word = Word::new("ね");
    pub static ref NA: Word = Word::new("な");
}