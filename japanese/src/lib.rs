pub use kumou_proto;

pub mod class;
pub mod particle;

pub mod pattern;

pub mod prelude {
    pub use kumou_proto::prelude::*;

    pub use crate::class::{WordClass, PatternClass};
    pub use crate::particle;
    pub use crate::pattern;
}