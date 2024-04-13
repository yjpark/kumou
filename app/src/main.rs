use dioxus::prelude::*;
use log::LevelFilter;

use kumou_app::app::App;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}
