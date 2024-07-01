# Cosmic-noodle

POC of waste management system

## requirements

- rust 1.78.7

## dependencies

- [onnxruntime](https://github.com/microsoft/onnxruntime/releases/tag/v1.17.3)

## references

- [extract Mat pixel from this tutorial](https://docs.opencv.org/4.x/de/d06/tutorial_js_basic_ops.html)
- [StableView](https://github.com/Shubhamai/StableView)
- [Implementing YOLOv8 Object Detection with OpenCV in Rust Using ONNX Models](https://linzichun.com/posts/rust-opencv-onnx-yolov8-detect/)

## issues

- opencv binding won't build on my main machine (Kubuntu 22.4), the error [fatal error: 'memory' file not found](https://github.com/apple/swift-nio-ssl/issues/105)

- nokhwa can't build artifacts on my second machine (Kubuntu 24.4) error: v4l2-sys-mit invalid indent
