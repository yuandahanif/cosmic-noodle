pub mod camera {
    use crossbeam_channel::Sender;
    use std::{
        collections::HashMap,
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        thread,
    };

    use nokhwa::{
        nokhwa_initialize,
        pixel_format::RgbFormat,
        query,
        utils::{ApiBackend, CameraIndex, RequestedFormat, RequestedFormatType},
        Buffer, CallbackCamera,
    };

    pub struct Camera {
        device_list: HashMap<String, CameraIndex>,
        selected_camera: Option<CameraIndex>,
        sender: Sender<Buffer>,
        keep_running: Arc<AtomicBool>,
        cam_thread: Option<thread::JoinHandle<()>>,
    }

    impl Camera {
        pub fn new(sender: Sender<Buffer>) -> Camera {
            Camera {
                device_list: HashMap::new(),
                selected_camera: None,
                keep_running: Arc::new(AtomicBool::new(false)),
                sender,
                cam_thread: None,
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

        pub fn get_selected_camera(&self) -> Option<&CameraIndex> {
            self.selected_camera.as_ref()
        }

        pub fn toggle_camera(&mut self) {
            if self.keep_running.load(Ordering::SeqCst) {
                self.shutdown();
            } else {
                self.start_camera();
            }
        }

        pub fn start_camera(&mut self) {
            // let camera_index = self
            //     .selected_camera
            //     .take()
            //     .or_else(|| self.device_list.values().next().cloned());
            // let camera_index = camera_index.unwrap();

            if self.cam_thread.is_some() {
                return;
            }

            let tx = self.sender.clone();
            self.keep_running.store(true, Ordering::SeqCst);
            let cloned_keep_running = self.keep_running.clone();

            let cameras = query(ApiBackend::Auto).unwrap();
            // nokwah bug on v4l2 backend
            let first_camera = cameras.get(1).unwrap();
            let format =
                RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
            let mut threaded =
                CallbackCamera::new(first_camera.index().clone(), format, |_buffer| {}).unwrap();

            let cam_thread = thread::spawn(move || {
                threaded.open_stream().unwrap();

                while cloned_keep_running.load(Ordering::SeqCst) {
                    if let Ok(frame) = threaded.poll_frame() {
                        println!("{:?}", frame.resolution());
                        if tx.send(frame).is_err() {
                            break;
                        }
                    }
                }

                threaded.stop_stream().unwrap();
            });

            self.cam_thread = Some(cam_thread);
        }

        pub fn shutdown(&mut self) {
            self.keep_running.store(false, Ordering::SeqCst);

            match self.cam_thread.take() {
                Some(cam_thread) => match cam_thread.join() {
                    Ok(_) => print!("thread stop"),
                    Err(error) => {
                        println!("Error joining camera thread: {:?}", error);
                    }
                },
                None => {
                    // panic!("Camera thread is not running");
                }
            }
        }
    }
}
