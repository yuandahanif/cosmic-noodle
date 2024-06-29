pub mod app {
    use iced::{
        advanced::mouse,
        executor,
        widget::{canvas, column, container, horizontal_space, row, scrollable},
        Alignment, Application, Command, Element, Length, Point, Rectangle, Renderer, Subscription,
        Theme,
    };

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
            let header = container(
                row![
                    square(40),
                    horizontal_space(),
                    "Header!",
                    horizontal_space(),
                    square(40),
                ]
                .padding(10)
                .align_items(Alignment::Center),
            );

            let sidebar = container(
                column!["Sidebar!", square(50), square(50)]
                    .spacing(40)
                    .padding(10)
                    .width(200)
                    .align_items(Alignment::Center),
            )
            .center_y();

            let content = container(
                scrollable(
                    column!["Content!", square(400), square(200), square(400), "The end"]
                        .spacing(40)
                        .align_items(Alignment::Center)
                        .width(Length::Fill),
                )
                .height(Length::Fill),
            )
            .padding(10);

            column![header, row![sidebar, content]].into()
        }
    }

    pub fn square<'a>(size: impl Into<Length> + Copy) -> Element<'a, Message> {
        struct Square;

        impl canvas::Program<Message> for Square {
            type State = ();

            fn draw(
                &self,
                _state: &Self::State,
                renderer: &Renderer,
                theme: &Theme,
                bounds: Rectangle,
                _cursor: mouse::Cursor,
            ) -> Vec<canvas::Geometry> {
                let mut frame = canvas::Frame::new(renderer, bounds.size());

                let palette = theme.extended_palette();

                frame.fill_rectangle(
                    Point::ORIGIN,
                    bounds.size(),
                    palette.background.strong.color,
                );

                vec![frame.into_geometry()]
            }
        }

        canvas(Square).width(size).height(size).into()
    }
}
