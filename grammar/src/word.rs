use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WordKind {
    Prefix,
    Suffix,
    Link,
}

pub trait Word {
    fn kind(&self) -> WordKind;
}