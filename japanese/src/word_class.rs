use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WordClass {
    Noun,
    Verb,
    Adjective,
    AdjectiveNoun,
    Adverb,
    Particle,
    Adnominal,
    Conjunction,
    Auxiliary,
    Interjection,
}

// 動詞、形容詞、形容動詞、名詞、副詞、連體詞、接續詞、感動詞、助動詞、助詞

