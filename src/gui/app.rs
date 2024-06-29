pub mod app {
    use iced::{executor, Application, Command, Element, Theme};

    use crate::gui::home::Message;

    pub struct config {
        pub name: String,
        pub version: String,
        pub author: String,
        pub qualifier: String,
    }

    impl Default for config {
        fn default() -> Self {
            config {
                name: String::from("app"),
                version: String::from("0.1.0"),
                author: String::from("author"),
                qualifier: String::from("com"),
            }
        }
    }

    pub struct App {
        config: config,
    }

    impl Application for App {
        type Executor = executor::Default;
        type Flags = ();
        type Message = Message;
        type Theme = Theme;

        fn new(_flags: ()) -> (App, Command<Message>) {
            (
                App {
                    config: config::default(),
                },
                Command::none(),
            )
        }

        fn title(&self) -> String {
            format!("{} v{}", self.config.name, self.config.version)
        }

        fn update(&mut self, message: Message) -> Command<Message> {
            // match message {
            //     Message::Start => {
            //         self.home.update(Message::Start);
            //     }
            //     Message::Pause => {
            //         self.home.update(Message::Pause);
            //     }
            // }

            Command::none()
        }

        fn view(&self) -> Element<Message> {
            // self.home.view()
            "Hello, world!".into()
        }
    }
}
