use dioxus::prelude::*;
use song_proto::prelude::*;

static TAB_BYTES: &'static [u8] = include_bytes!("../../assets/tabs/anne.ron");

pub static TAB: GlobalSignal<LoadTabResult> = Signal::global(|
| {
    load_tab(TAB_BYTES)
});