pub mod consts {

    pub const APP_NAME: &str = "cosmic-noodle";
    pub const AUTHOR: &str = "yume";
    pub const QUALIFIER: &str = "rs";
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");

    pub const MODEL_TACO: &[u8] = include_bytes!("../assets/model/yolov8-taco.onnx");
    pub const MODEL_TACO_CLASSES: &[u8] =
        include_bytes!("../assets/model/yolov8-taco-classes.yaml");

    pub const INTER_FONT: &[u8] = include_bytes!("../assets/fonts/Inter-Regular.ttf");

    pub const YOLO_CLASS: [&str; 60] = [
        "Aluminium foil",
        "Battery",
        "Aluminium blister pack",
        "Carded blister pack",
        "Other plastic bottle",
        "Clear plastic bottle",
        "Glass bottle",
        "Plastic bottle cap",
        "Metal bottle cap",
        "Broken glass",
        "Food Can",
        "Aerosol",
        "Drink can",
        "Toilet tube",
        "Other carton",
        "Egg carton",
        "Drink carton",
        "Corrugated carton",
        "Meal carton",
        "Pizza box",
        "Paper cup",
        "Disposable plastic cup",
        "Foam cup",
        "Glass cup",
        "Other plastic cup",
        "Food waste",
        "Glass jar",
        "Plastic lid",
        "Metal lid",
        "Other plastic",
        "Magazine paper",
        "Tissues",
        "Wrapping paper",
        "Normal paper",
        "Paper bag",
        "Plastified paper bag",
        "Plastic film",
        "Six pack rings",
        "Garbage bag",
        "Other plastic wrapper",
        "Single-use carrier bag",
        "Polypropylene bag",
        "Crisp packet",
        "Spread tub",
        "Tupperware",
        "Disposable food container",
        "Foam food container",
        "Other plastic container",
        "Plastic glooves",
        "Plastic utensils",
        "Pop tab",
        "Rope & strings",
        "Scrap metal",
        "Shoe",
        "Squeezable tube",
        "Plastic straw",
        "Paper straw",
        "Styrofoam piece",
        "Unlabeled litter",
        "Cigarette",
    ];
}
