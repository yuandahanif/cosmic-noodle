#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use directories::ProjectDirs;

mod consts;
mod gui;

use consts::consts::{self as CONST, INTER_FONT};
use gui::app::app::{App, Config, Flags};
use iced::{
    window::{self, settings::PlatformSpecific, Level},
    Application, Settings, Size,
};

fn main() -> iced::Result {
    if let Some(proj_dirs) = ProjectDirs::from(CONST::QUALIFIER, CONST::AUTHOR, CONST::APP_NAME) {
        let dir = proj_dirs.config_dir();

        println!("{:?}", dir.to_str());
    }

    let flags = Flags {
        config: Config::new(
            CONST::APP_NAME.to_string(),
            CONST::VERSION.to_string(),
            CONST::AUTHOR.to_string(),
            CONST::QUALIFIER.to_string(),
        ),
    };

    let settings = Settings {
        id: None,
        window: window::Settings {
            size: Size::new(750., 620.), // start size
            position: window::Position::Centered,
            min_size: Some(Size::new(750., 620.)),
            max_size: None,
            resizable: true,
            decorations: true,
            transparent: false,
            icon: None,
            visible: true,
            platform_specific: PlatformSpecific::default(),
            exit_on_close_request: true,
            level: Level::Normal,
        },
        flags,
        default_font: iced::font::Font::with_name("Inter-Regular"),
        fonts: vec![std::borrow::Cow::Borrowed(INTER_FONT)],
        default_text_size: iced::Pixels(13.),
        antialiasing: true,
    };

    App::run(settings)
}
