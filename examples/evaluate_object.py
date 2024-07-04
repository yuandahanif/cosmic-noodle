import os
import cv2
import numpy as np
import onnxruntime as ort
from tqdm import tqdm
from collections import defaultdict
import torch

# Model and dataset paths
MODEL_PATH = "../assets/model/yolov8-taco.onnx"
DATASET_PATH = "../assets/test"
# DATASET_PATH = "/home/yume/coding/waste-management/val"
CLASS_NAMES_PATH = "../assets/model/class_name.txt"


# Load the ONNX model
providers = [("CUDAExecutionProvider", {"device_id": torch.cuda.current_device(),
                                        "user_compute_stream": str(torch.cuda.current_stream().cuda_stream)})]
sess_options = ort.SessionOptions()

ort_session = ort.InferenceSession(MODEL_PATH, sess_options=sess_options, providers=providers)
input_name = ort_session.get_inputs()[0].name
input_shape = ort_session.get_inputs()[0].shape

#  Load class names
with open(CLASS_NAMES_PATH, "r") as f:
    class_names = [line.strip() for line in f.readlines()]

def preprocess_image(image, input_shape):
    height, width = input_shape[2], input_shape[3]
    image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
    image = cv2.resize(image, (width, height))
    image = image.transpose((2, 0, 1))
    image = np.expand_dims(image, axis=0)
    image = image.astype(np.float32) / 255.0
    return image

def postprocess_output(output, image_shape, conf_threshold=0.5):
    height, width = image_shape[:2]
    boxes = output[0].transpose()
    
    outputs = []
    for row in boxes:
        class_id = np.argmax(row[4:])
        prob = row[class_id + 4]
        
        # if prob < conf_threshold:
        #     continue
        
        label = class_names[class_id]
        xc, yc, w, h = row[:4]
        
        xc = xc / 640.0 * width
        yc = yc / 640.0 * height
        w = w / 640.0 * width
        h = h / 640.0 * height
        
        x1 = xc - w / 2
        y1 = yc - h / 2
        x2 = xc + w / 2
        y2 = yc + h / 2
        
        outputs.append((x1, y1, x2, y2, label, prob))
    
    return outputs

def evaluate_model():
    total_images = 0
    correct_predictions = 0
    class_metrics = defaultdict(lambda: {"correct": 0, "total": 0})

    for class_folder in os.listdir(DATASET_PATH):
        class_path = os.path.join(DATASET_PATH, class_folder)
        if not os.path.isdir(class_path):
            continue

        print(f"Processing {class_folder}...")
        for image_file in tqdm(os.listdir(class_path)):
            image_path = os.path.join(class_path, image_file)
            image = cv2.imread(image_path)
            if image is None:
                continue

            total_images += 1
            class_metrics[class_folder]["total"] += 1

            input_tensor = preprocess_image(image, input_shape)
            outputs = ort_session.run(None, {input_name: input_tensor})
            detections = postprocess_output(outputs, image.shape)

            if detections:
                # Sort detections by confidence and get the top prediction
                top_prediction = max(detections, key=lambda x: x[5])
                predicted_class = top_prediction[4]  # The label is at index 4
                
                if predicted_class == class_folder:
                    correct_predictions += 1
                    class_metrics[class_folder]["correct"] += 1

    overall_accuracy = correct_predictions / total_images if total_images > 0 else 0
    print(f"\nOverall Accuracy: {overall_accuracy:.2%}")

    print("\nClass-wise Accuracy:")
    for class_name, metrics in class_metrics.items():
        class_accuracy = metrics["correct"] / metrics["total"] if metrics["total"] > 0 else 0
        print(f"{class_name}: {class_accuracy:.2%} ({metrics['correct']}/{metrics['total']})")

if __name__ == "__main__":
    evaluate_model()