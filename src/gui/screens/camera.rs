use iced::{
    widget::{
        button, column, container,
        image::{self, viewer},
        pick_list, row, text,
    },
    Element, Length,
};

use opencv::{core::VectorToVec, imgcodecs};

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

    let mut image_viewer: image::Viewer<image::Handle> = viewer(image::Handle::from_memory(vec![]))
        .width(Length::Fill)
        .height(Length::Fixed(200.));

    if let Ok(frame) = app.cam_rx.try_recv() {
        let mut encoded_image = opencv::core::Vector::<u8>::new();
        let params = opencv::core::Vector::<i32>::new();
        match imgcodecs::imencode(".PNG", &frame, &mut encoded_image, &params) {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("Error encoding image: {}", e);
            }
        }
        let image = encoded_image.to_vec();

        image_viewer = viewer(image::Handle::from_memory(image))
            .width(Length::Fill)
            .height(Length::Fixed(200.));
    }

    container(
        column![
            image_viewer,
            container(row!(
                button("Toggle Camera").on_press(Message::CameraToggle),
                text("Select a camera:"),
                camera_pick_list,
            ))
            .width(Length::Fill)
        ]
        .spacing(40)
        .width(Length::Fill),
    )
    .height(Length::Fill)
    .into()
}
