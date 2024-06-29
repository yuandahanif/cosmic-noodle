pub mod consts {
    pub const APP_NAME: &str = "cosmic-noodle";
    pub const AUTHOR: &str = "yume";
    pub const QUALIFIER: &str = "rs";

    pub const MODEL_TACO: &[u8] = include_bytes!("../assets/model/yolov8-taco.onnx");
    pub const MODEL_TACO_CLASSES: &[u8] =
        include_bytes!("../assets/model/yolov8-taco-classes.yaml");
}
