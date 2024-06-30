pub mod camera {
    use std::collections::HashMap;

    use nokhwa::{
        nokhwa_initialize,
        pixel_format::{RgbAFormat, RgbFormat},
        query,
        utils::{ApiBackend, CameraIndex, RequestedFormat, RequestedFormatType},
        CallbackCamera,
    };

    pub struct Camera {
        device_list: HashMap<String, CameraIndex>,
        selected_camera: Option<CameraIndex>,
        thread_camera: Option<CallbackCamera>,
    }

    impl Camera {
        pub fn new() -> Camera {
            Camera {
                device_list: HashMap::new(),
                selected_camera: None,
                thread_camera: None,
            }
        }

        pub fn get_available_cameras(&mut self) {
            nokhwa_initialize(|granted| {
                println!("User said {}", granted);
            });
            self.device_list.clear();

            let cameras = query(ApiBackend::Auto).unwrap();
            cameras.iter().for_each(|cam| {
                let index = cam.index().clone();

                self.device_list
                    .insert(format!("{:<4} {}", cam.human_name(), cam.index()), index);
            });
        }

        pub fn get_camera_list(&self) -> &HashMap<String, CameraIndex> {
            &self.device_list
        }

        pub fn select_camera(&mut self, index: CameraIndex) {
            self.selected_camera = Some(index);
        }

        pub fn start_camera_thread(&mut self) {
            if let Some(selected_camera) = &self.selected_camera {
                let format = RequestedFormat::new::<RgbFormat>(
                    RequestedFormatType::AbsoluteHighestFrameRate,
                );
                let threaded = CallbackCamera::new(selected_camera.clone(), format, |buffer| {
                    let image = buffer.decode_image::<RgbAFormat>().unwrap();
                    println!("{}x{} {}", image.width(), image.height(), image.len());
                })
                .unwrap();

                self.thread_camera = Some(threaded);
            } else {
                println!("No camera selected");
            }
        }

        pub fn start_camera_stream(&mut self) {
            let mut threaded = self.thread_camera.take().expect("No camera thread");
            threaded.open_stream().unwrap();
            self.thread_camera = Some(threaded);
        }

        pub fn stop_camera_thread(&mut self) {
            let mut threaded = self.thread_camera.take().expect("No camera thread");
            threaded.stop_stream().unwrap();
            self.thread_camera = Some(threaded);
        }

        pub fn get_stream(&self) -> Option<&CallbackCamera> {
            self.thread_camera.as_ref()
        }
    }
}
