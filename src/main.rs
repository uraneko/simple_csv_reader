pub mod csv;
pub mod gui;
pub mod matcell;

use crate::csv::reader::*;
use crate::gui::app::App;
use iced::{Application, Settings};

// fn main() {
//     let t = full_read(SAMPLE_FILE);
//     println!("{:?}", t);
// }

fn main() -> iced::Result {
    App::run(Settings::default())
}
