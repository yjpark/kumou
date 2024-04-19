pub use kumou_proto;

pub mod word_class;
pub mod particle;

pub mod prelude {
    pub use kumou_proto::prelude::*;

    pub use crate::word_class::WordClass;
    pub use crate::particle;
}