pub mod onnx_thread {
    use std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        thread,
    };

    pub struct OnnxThread {
        pub thread: Option<std::thread::JoinHandle<()>>,
        keep_running: Arc<AtomicBool>,
    }

    impl OnnxThread {
        pub fn new(session: crate::onnx::onnx_session::onnx_session::OnnxSession) -> Self {
            let keep_running = Arc::new(AtomicBool::new(true));
            let keep_runing_clone = keep_running.clone();

            let thread = thread::spawn(move || loop {
                if !keep_runing_clone.load(Ordering::SeqCst) {
                    break;
                }
                session.run_with_sender();
            });

            OnnxThread {
                thread: Some(thread),
                keep_running,
            }
        }

        pub fn toggle(&mut self) {
            if self.keep_running.load(Ordering::SeqCst) {
                self.stop();
            } else {
                self.start();
            }
        }

        pub fn start(&mut self) {
            self.keep_running.store(true, Ordering::SeqCst);
        }

        pub fn stop(&mut self) {
            self.keep_running.store(false, Ordering::SeqCst);
            match self.thread.take() {
                Some(cam_thread) => match cam_thread.join() {
                    Ok(_) => print!("thread stop"),
                    Err(error) => {
                        println!("Error joining onnx thread: {:?}", error);
                    }
                },
                None => {
                    println!("onnx thread is not running!");
                }
            }
        }
    }
}
