use iced::{
    widget::{
        button, column, container,
        image::{self, viewer},
        pick_list, row, text,
    },
    Element, Length,
};

use opencv::{
    core::{MatTraitConst, VectorToVec},
    imgcodecs,
};

use crate::gui::app::app::{App, Message};

pub fn camera_screen<'a>(app: &'a App) -> Element<'a, Message> {
    let camera_list = app.camera.get_camera_list();
    let oprions = camera_list.values().cloned().collect::<Vec<_>>();

    let camera_pick_list = pick_list(
        oprions,
        app.camera.get_selected_camera(),
        Message::SelectCamera,
    )
    .placeholder("Choose webcam");

    let frame = app.state.frame.clone();
    let mut encoded_image = opencv::core::Vector::<u8>::new();
    let params = opencv::core::Vector::<i32>::new();
    encoded_image.clear();

    if !frame.empty() {
        match imgcodecs::imencode(".png", &frame, &mut encoded_image, &params) {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("Error encoding image: {}", e);
            }
        }
    }

    let image = encoded_image.to_vec();

    let image_viewer: image::Viewer<image::Handle> = viewer(image::Handle::from_memory(image))
        .width(Length::Fill)
        .height(Length::Fixed(300.));

    let prediction_label = app.state.prediction.iter().fold(String::new(), |acc, x| {
        format!("{}{}: {:.2}\n", acc, x.4, x.5)
    });

    container(
        column![
            text(app.state.tick.to_string())
                .size(50)
                .width(Length::Fill),
            image_viewer,
            container(row!(
                button("Toggle Camera").on_press(Message::CameraToggle),
                text("Select a camera:"),
                camera_pick_list,
            ))
            .width(Length::Fill),
            text(prediction_label)
                .size(50)
                .width(Length::Fill),
        ]
        .spacing(40)
        .width(Length::Fill),
    )
    .height(Length::Fill)
    .into()
}
