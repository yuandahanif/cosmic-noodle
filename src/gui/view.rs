use iced::{
    advanced::mouse,
    widget::{button, canvas, column, container, horizontal_space, row, scrollable, text, Column},
    Alignment, Element, Length, Point, Rectangle, Renderer, Theme,
};

use crate::gui::app::app::{App, Message, Screen};
use crate::gui::screens::system_information::system_information_screen;

use super::screens::camera::camera_screen;

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

pub fn app_view(app: &App) -> Column<Message> {
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
        column![
            "Sidebar!",
            button("Home").on_press(Message::Navigate(Screen::Home)),
            button("Camera").on_press(Message::Navigate(Screen::Camera)),
            button("System Information").on_press_maybe(match &app.state.system_information {
                Some(inf) => Some(Message::Navigate(Screen::SystemInformation(inf.clone()))),
                None => None,
            }),
        ]
        .spacing(10)
        .padding(10)
        .width(200)
        .align_items(Alignment::Start),
    )
    .center_y();

    let content = match &app.screen {
        Screen::SystemInformation(information) => container(
            scrollable(
                column![
                    "System information!",
                    system_information_screen(information.clone())
                ]
                .spacing(40)
                .align_items(Alignment::Center)
                .width(Length::Fill),
            )
            .height(Length::Fill),
        ),
        Screen::Home => container(
            scrollable(
                column!["Home!",]
                    .spacing(40)
                    .align_items(Alignment::Center)
                    .width(Length::Fill),
            )
            .height(Length::Fill),
        ),
        Screen::Settings => {
            todo!()
        }
        Screen::Camera => container(
            scrollable(
                column!["Camera!", camera_screen(&app)]
                    .spacing(40)
                    .width(Length::Fill),
            )
            .height(Length::Fill),
        ),
    };

    column![header, row![sidebar, content]].into()
}
