pub mod onnx_session {
    use ndarray::{s, ArrayBase, Axis, Dim, IxDynImpl, ViewRepr};
    use ort::{GraphOptimizationLevel, Session};

    use opencv::{
        core::{MatExprTraitConst, MatTraitConst, MatTraitConstManual},
        prelude::Mat,
    };

    use crate::{consts::consts::YOLO_CLASS, onnx::iou::iou};

    pub struct OnnxSession {
        session: Session,
    }

    impl OnnxSession {
        pub fn new(model: &[u8]) -> Self {
            let session = Session::builder()
                .unwrap()
                .with_optimization_level(GraphOptimizationLevel::Level3)
                .unwrap()
                .with_intra_threads(4)
                .unwrap()
                .commit_from_memory(model)
                .unwrap();

            OnnxSession { session }
        }

        fn pre_process(&self, image: &Mat) -> opencv::Result<Mat> {
            let width = image.cols();
            let height = image.rows();

            let _max = std::cmp::max(width, height);
            // keep the original aspect ratio by adding black padding
            let mut result = Mat::zeros(_max, _max, opencv::core::CV_32FC1)
                .unwrap()
                .to_mat()
                .unwrap();
            image.copy_to(&mut result)?;

            let size = opencv::core::Size::new(640, 640);
            let mut resized_mat = Mat::zeros(_max, _max, opencv::core::CV_32FC1)
                .unwrap()
                .to_mat()
                .unwrap();
            opencv::imgproc::resize(
                &result,
                &mut resized_mat,
                size,
                0.0,
                0.0,
                opencv::imgproc::INTER_LINEAR,
            )?;

            Ok(resized_mat)
        }

        fn post_process(
            &self,
            outs: ArrayBase<ViewRepr<&f32>, Dim<IxDynImpl>>,
            size: (i32, i32),
            conf_thresh: f32,
            nms_thresh: f32,
        ) -> Vec<(f32, f32, f32, f32, std::string::String, f32)> {
            let width = size.0;
            let height = size.1;
            let boxes = outs.t();
            let boxes = boxes.slice(s![.., .., 0]);
            let mut outputs = Vec::new();

            for row in boxes.axis_iter(Axis(0)) {
                let row: Vec<_> = row.iter().map(|x| *x).collect();
                let (class_id, prob) = row
                    .iter()
                    .skip(4)
                    .enumerate()
                    .map(|(index, value)| (index, *value))
                    .reduce(|accum, row| if row.1 > accum.1 { row } else { accum })
                    .unwrap();

                if prob < conf_thresh {
                    continue;
                }

                let label = YOLO_CLASS[class_id].to_string();
                let xc = row[0] / 640.0 * (width as f32);
                let yc = row[1] / 640.0 * (height as f32);
                let w = row[2] / 640.0 * (width as f32);
                let h = row[3] / 640.0 * (height as f32);
                let x1 = xc - w / 2.0;
                let x2 = xc + w / 2.0;
                let y1 = yc - h / 2.0;
                let y2 = yc + h / 2.0;

                outputs.push((x1, y1, x2, y2, label, prob));
            }

            let mut result = Vec::new();
            outputs.sort_by(|box1, box2| box2.5.total_cmp(&box1.5));
            while outputs.len() > 0 {
                result.push(outputs[0].clone());
                outputs = outputs
                    .iter()
                    .filter(|box1| iou(&outputs[0], box1) < nms_thresh)
                    .map(|x| x.clone())
                    .collect()
            }

            return result;
        }

        pub fn run(&self, image: Mat) -> ort::Result<Vec<(f32, f32, f32, f32, String, f32)>> {
            let width = image.cols();
            let height = image.rows();

            let src = self.pre_process(&image).unwrap();
            let mut input = ndarray::Array::zeros((1, 3, 640, 640)).into_dyn();

            if src.is_continuous() {
                let size = src.size().unwrap();
                let data = src.data_bytes().unwrap();

                for row in 0..size.height {
                    for col in 0..size.width {
                        let x = row as usize;
                        let y = col as usize;
                        let r = data
                            [(row * src.cols() * src.channels() + col * src.channels()) as usize];
                        let g = data[(row * src.cols() * src.channels() + col * src.channels() + 1)
                            as usize];
                        let b = data[(row * src.cols() * src.channels() + col * src.channels() + 2)
                            as usize];
                        // let a = data[(row * src.cols() * src.channels() + col * src.channels() + 3)
                        //     as usize];
                        input[[0, 0, y, x]] = (r as f32) / 255.0;
                        input[[0, 1, y, x]] = (g as f32) / 255.0;
                        input[[0, 2, y, x]] = (b as f32) / 255.0;
                    }
                }
            }

            let outputs = self
                .session
                .run(ort::inputs!["images" => input.view()]?)
                .unwrap();
            let output = outputs
                .get("output0")
                .unwrap()
                .try_extract_tensor::<f32>()?;

            let result = self.post_process(output, (width, height), 0.4, 0.3);
            Ok(result)
        }
    }
}
