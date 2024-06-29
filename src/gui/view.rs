use iced::{
    advanced::mouse,
    widget::{canvas, column, container, horizontal_space, row, scrollable, Column},
    Alignment, Element, Length, Point, Rectangle, Renderer, Theme,
};

use crate::gui::app::app::{App, Message};

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
