#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use consts::consts::{self as CONST, INTER_FONT, MODEL_TACO};
use crossbeam_channel::unbounded;
use directories::ProjectDirs;
use iced::{
    window::{self, settings::PlatformSpecific, Level},
    Application, Settings, Size,
};
use onnx::onnx_thread;
use opencv::prelude::Mat;
use tracing::Level as TraceLevel;
use tracing_subscriber::FmtSubscriber;

use gui::{
    app::app::{App, Flags},
    config::Config,
};
use types::custom_type::BoundingBoxResult;

mod camera;
mod consts;
mod gui;
mod onnx;
mod types;

fn main() -> iced::Result {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(TraceLevel::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    if let Some(proj_dirs) = ProjectDirs::from(CONST::QUALIFIER, CONST::AUTHOR, CONST::APP_NAME) {
        let dir = proj_dirs.config_dir();

        println!("{:?}", dir.to_str());
    }

    let (cam_tx, cam_rx) = unbounded::<Mat>();
    let camera = camera::camera::Camera::new(cam_tx);

    let (model_output_tx, model_output_rx) = unbounded::<Vec<BoundingBoxResult>>();
    let (model_input_tx, model_input_rx) = unbounded::<Mat>();
    let onnx_session = onnx::onnx_session::onnx_session::OnnxSession::new(
        MODEL_TACO,
        model_output_tx,
        model_input_rx,
    );
    let onnx_thread = onnx_thread::onnx_thread::OnnxThread::new(onnx_session);

    let flags = Flags {
        config: Config::new(
            CONST::APP_NAME.to_string(),
            CONST::VERSION.to_string(),
            CONST::AUTHOR.to_string(),
            CONST::QUALIFIER.to_string(),
        ),
        camera,
        cam_rx,
        model_output_rx,
        model_input_tx,
        onnx_thread,
    };

    let settings = Settings {
        id: None,
        window: window::Settings {
            size: Size::new(1352., 755.), // start size
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
