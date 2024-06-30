use bytesize::ByteSize;
use iced::{
    advanced::mouse,
    system,
    widget::{canvas, column, container, horizontal_space, row, scrollable, text, Column},
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

fn system_information(information: system::Information) -> Element<'static, Message> {
    let system_name = text(format!(
        "System name: {}",
        information
            .system_name
            .as_ref()
            .unwrap_or(&"unknown".to_string())
    ));

    let system_kernel = text(format!(
        "System kernel: {}",
        information
            .system_kernel
            .as_ref()
            .unwrap_or(&"unknown".to_string())
    ));

    let system_version = text(format!(
        "System version: {}",
        information
            .system_version
            .as_ref()
            .unwrap_or(&"unknown".to_string())
    ));

    let system_short_version = text(format!(
        "System short version: {}",
        information
            .system_short_version
            .as_ref()
            .unwrap_or(&"unknown".to_string())
    ));

    let cpu_brand = text(format!("Processor brand: {}", information.cpu_brand));

    let cpu_cores = text(format!(
        "Processor cores: {}",
        information
            .cpu_cores
            .map_or("unknown".to_string(), |cores| cores.to_string())
    ));

    let memory_readable = ByteSize::b(information.memory_total).to_string();

    let memory_total = text(format!(
        "Memory (total): {} bytes ({memory_readable})",
        information.memory_total,
    ));

    let memory_text = if let Some(memory_used) = information.memory_used {
        let memory_readable = ByteSize::b(memory_used).to_string();

        format!("{memory_used} bytes ({memory_readable})")
    } else {
        String::from("None")
    };

    let memory_used = text(format!("Memory (used): {memory_text}"));

    let graphics_adapter = text(format!(
        "Graphics adapter: {}",
        information.graphics_adapter
    ));

    let graphics_backend = text(format!(
        "Graphics backend: {}",
        information.graphics_backend
    ));

    column![
        system_name.size(30),
        system_kernel.size(30),
        system_version.size(30),
        system_short_version.size(30),
        cpu_brand.size(30),
        cpu_cores.size(30),
        memory_total.size(30),
        memory_used.size(30),
        graphics_adapter.size(30),
        graphics_backend.size(30),
        // button("Refresh").on_press(Message::Refresh)
    ]
    .spacing(10)
    .into()
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

    let system_information = match &app.system_information {
        Some(information) => system_information(information.clone()),
        None => column!["No system information"].into(),
    };

    let content = container(
        scrollable(
            column![
                "Content!",
                text(app.state.tick).size(10),
                system_information,
                square(200),
                square(400),
                "The end"
            ]
            .spacing(40)
            .align_items(Alignment::Center)
            .width(Length::Fill),
        )
        .height(Length::Fill),
    )
    .padding(10);

    column![header, row![sidebar, content]].into()
}
