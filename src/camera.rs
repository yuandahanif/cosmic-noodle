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

    use opencv::{
        prelude::{Mat, VideoCaptureTrait, VideoCaptureTraitConst},
        videoio,
    };

    pub struct Camera {
        device_list: HashMap<String, i32>,
        selected_camera: Option<i32>,
        sender: Sender<Mat>,
        keep_running: Arc<AtomicBool>,
        cam_thread: Option<thread::JoinHandle<()>>,
    }

    impl Camera {
        pub fn new(sender: Sender<Mat>) -> Camera {
            Camera {
                device_list: HashMap::new(),
                selected_camera: None,
                keep_running: Arc::new(AtomicBool::new(false)),
                sender,
                cam_thread: None,
            }
        }

        pub fn get_available_cameras(&mut self) {
            todo!("Get list of available cameras")
        }

        pub fn get_camera_list(&self) -> &HashMap<String, i32> {
            &self.device_list
        }

        pub fn select_camera(&mut self, index: i32) {
            self.selected_camera = Some(index);
        }

        pub fn get_selected_camera(&self) -> Option<&i32> {
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
            if self.cam_thread.is_some() {
                return;
            }
            let _camera_index = self
                .selected_camera
                .take()
                .or_else(|| self.device_list.values().next().cloned());

            // FIXME: This is a hack to get the camera index
            let mut cam = match videoio::VideoCapture::new(0, videoio::CAP_ANY) {
                Ok(cam) => cam,
                Err(error) => {
                    println!("Error opening camera: {:?}", error);
                    return;
                }
            };

            let opened = match videoio::VideoCapture::is_opened(&cam) {
                Ok(opened) => opened,
                Err(error) => {
                    println!("Error checking if camera is opened: {:?}", error);
                    return;
                }
            };

            if opened {
                println!("Camera is opened");
            } else {
                println!("Camera is not opened");
                return;
            }

            let tx = self.sender.clone();
            self.keep_running.store(true, Ordering::SeqCst);
            let cloned_keep_running = self.keep_running.clone();

            let cam_thread = thread::spawn(move || {
                // Running loop as long as keep_running is true
                while cloned_keep_running.load(Ordering::SeqCst) {
                    // Reading frame
                    let mut frame = Mat::default();
                    match cam.read(&mut frame) {
                        Ok(_) => (),
                        Err(error) => {
                            panic!("Unable to read frame from camera : {:?}", error);
                        }
                    }

                    if tx.send(frame).is_err() {
                        break;
                    }
                }
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
