pub mod app {
    use iced::system;
    use iced::{
        executor, widget::Container, Application, Command, Element, Length, Subscription, Theme,
    };

    use crate::camera::camera::Camera;
    use crate::gui::{config::Config, view::app_view};

    #[derive(Debug, Default)]
    pub struct State {
        pub tick: u64,
        pub system_information: Option<system::Information>,
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
    }

    pub struct Flags {
        pub config: Config,
        pub camera: Camera,
    }

    #[derive(Debug, Clone)]
    pub enum Message {
        Tick,
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
                }
                Message::SystemInformationReceived(information) => {
                    self.state.system_information = Some(information);
                }
                Message::Navigate(screen) => {
                    self.screen = screen;
                }
            }

            Command::none()
        }

        fn subscription(&self) -> Subscription<Message> {
            iced::time::every(std::time::Duration::from_millis(500)).map(|_| Message::Tick)
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
