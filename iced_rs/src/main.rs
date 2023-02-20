use iced::{Application, Settings};

//static WINDOW_ICON: &[u8] = include_bytes!("../eye.ico");

mod ui;

pub fn main() -> iced::Result {
    let settings = Settings::default();
    return ui::KeyTuber::run(settings);
}
