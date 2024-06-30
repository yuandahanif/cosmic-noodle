pub mod app {
    use iced::{
        executor, widget::Container, Application, Command, Element, Length, Subscription, Theme,
    };

    use crate::gui::view::app_view;

    pub struct Config {
        name: String,
        version: String,
        author: String,
        qualifier: String,
    }

    impl Default for Config {
        fn default() -> Self {
            Config {
                name: String::from("app"),
                version: String::from("0.1.0"),
                author: String::from("author"),
                qualifier: String::from("com"),
            }
        }
    }

    impl Config {
        pub fn new(name: String, version: String, author: String, qualifier: String) -> Self {
            Config {
                name,
                version,
                author,
                qualifier,
            }
        }
    }

    pub struct App {
        config: Config,
    }

    pub struct Flags {
        pub config: Config,
        pub camera: crate::camera::camera::Camera,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Message {
        Tick,
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
                },
                Command::none(),
            )
        }

        fn title(&self) -> String {
            format!("{} v{}", self.config.name, self.config.version)
        }

        fn theme(&self) -> Theme {
            Theme::Light
        }

        fn update(&mut self, message: Message) -> Command<Message> {
            match message {
                Message::Tick => {}
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
