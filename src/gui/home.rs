use iced::Element;

#[derive(Default)]
pub enum Home {
    #[default]
    Loading,
    Paused,
    Running,
}

#[derive(Clone, Debug)]
pub enum Message {
    Start,
    Pause,
}

impl Home {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Start => {
                *self = Home::Running;
            }
            Message::Pause => {
                *self = Home::Paused;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        "Hello, world!".into()
    }
}
