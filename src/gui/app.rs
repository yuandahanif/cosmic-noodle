pub mod app {
    use crossbeam_channel::Receiver;
    use iced::system;
    use iced::{
        executor, widget::Container, Application, Command, Element, Length, Subscription, Theme,
    };

    use opencv::prelude::Mat;

    use crate::camera::camera::Camera;
    use crate::gui::{config::Config, view::app_view};
    use crate::onnx::onnx_session::onnx_session::OnnxSession;

    #[derive(Debug, Default)]
    pub struct State {
        pub tick: u64,
        pub system_information: Option<system::Information>,
        pub frame: Mat,
        pub prediction: Vec<(f32, f32, f32, f32, String, f32)>,
    }

    #[derive(Debug, Clone)]
    pub enum Screen {
        Home,
        Camera,
        SystemInformation(system::Information),
        Settings,
    }

    pub struct App {
        config: Config,
        pub camera: Camera,
        pub state: State,
        pub screen: Screen,
        pub cam_rx: Receiver<Mat>,
        pub onnx_session: OnnxSession,
    }

    pub struct Flags {
        pub config: Config,
        pub camera: Camera,
        pub cam_rx: Receiver<Mat>,
        pub onnx_session: OnnxSession,
    }

    #[derive(Debug, Clone)]
    pub enum Message {
        Tick,
        CameraToggle,
        SelectCamera(i32),
        SystemInformationReceived(system::Information),
        Navigate(Screen),
    }

    impl Application for App {
        type Executor = executor::Default;
        type Flags = Flags;
        type Message = Message;
        type Theme = Theme;

        fn new(flags: Flags) -> (App, Command<Message>) {
            (
                App {
                    config: flags.config,
                    camera: flags.camera,
                    state: State::default(),
                    screen: Screen::Home,
                    cam_rx: flags.cam_rx,
                    onnx_session: flags.onnx_session,
                },
                system::fetch_information(Message::SystemInformationReceived),
            )
        }

        fn title(&self) -> String {
            format!("{} v{}", self.config.name(), self.config.version())
        }

        fn theme(&self) -> Theme {
            Theme::Light
        }

        fn update(&mut self, message: Message) -> Command<Message> {
            match message {
                Message::Tick => {
                    self.state.tick = self.state.tick.wrapping_add(1);
                    self.state.frame = match self.cam_rx.try_recv() {
                        Ok(result) => {
                            // run the model every 10 frames
                            if self.state.tick % 100 == 0 {
                                let bounding = self.onnx_session.run(result.clone()).unwrap();

                                println!("{:?}", bounding);

                                self.state.prediction = bounding;
                                self.state.tick = 0;
                            }
                            result
                        }
                        Err(_) => self.state.frame.clone(),
                    };
                }
                Message::SystemInformationReceived(information) => {
                    self.state.system_information = Some(information);
                }
                Message::Navigate(screen) => {
                    self.screen = screen;
                }
                Message::SelectCamera(_) => todo!(),
                Message::CameraToggle => {
                    self.camera.toggle_camera();
                }
            }

            Command::none()
        }

        fn subscription(&self) -> Subscription<Message> {
            iced::time::every(std::time::Duration::from_millis(10)).map(|_| Message::Tick)
        }

        fn view(&self) -> Element<Message> {
            let body = app_view(self);

            Container::new(body)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
        }
    }
}
