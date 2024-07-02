import cv2
import numpy as np
import onnxruntime as ort

# Initialize webcam
cap = cv2.VideoCapture(0)

# Initialize ONNX Runtime
ort_session = ort.InferenceSession("../assets/model/yolov8-taco.onnx")

# Get model input details
model_inputs = ort_session.get_inputs()
input_shape = model_inputs[0].shape
input_name = model_inputs[0].name

# Load class labels
with open("../assets/model/class_name.txt", "r") as f:
    class_names = [line.strip() for line in f.readlines()]

def preprocess_image(image, input_shape):
    height, width = input_shape[2], input_shape[3]
    image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
    image = cv2.resize(image, (width, height))
    image = image.transpose((2, 0, 1))
    image = np.expand_dims(image, axis=0)
    image = image.astype(np.float32) / 255.0
    return image

def iou(box1, box2):
    x1, y1, x2, y2, _, _ = box1
    x1_, y1_, x2_, y2_, _, _ = box2
    
    xi1, yi1 = max(x1, x1_), max(y1, y1_)
    xi2, yi2 = min(x2, x2_), min(y2, y2_)
    
    inter_area = max(xi2 - xi1, 0) * max(yi2 - yi1, 0)
    box1_area = (x2 - x1) * (y2 - y1)
    box2_area = (x2_ - x1_) * (y2_ - y1_)
    
    return inter_area / (box1_area + box2_area - inter_area)

def postprocess_output(output, image_shape, conf_threshold=0.5, nms_threshold=0.5):
    height, width = image_shape[:2]
    boxes = output[0].transpose()  # Transpose to match Rust code structure
    
    outputs = []
    for row in boxes:
        class_id = np.argmax(row[4:])
        prob = row[class_id + 4]
        
        if prob < conf_threshold:
            continue
        
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
    
    # Sort by confidence
    outputs.sort(key=lambda x: x[5], reverse=True)
    
    # Apply NMS
    result = []
    while outputs:
        result.append(outputs[0])
        outputs = [box for box in outputs if iou(outputs[0], box) < nms_threshold]

    return result

while True:
    ret, frame = cap.read()
    if not ret:
        break

    input_tensor = preprocess_image(frame, input_shape)
    outputs = ort_session.run(None, {input_name: input_tensor})

    detections = postprocess_output(outputs, frame.shape)

    for x1, y1, x2, y2, label, prob in detections:
        x1, y1, x2, y2 = int(x1), int(y1), int(x2), int(y2)
        cv2.rectangle(frame, (x1, y1), (x2, y2), (0, 255, 0), 2)
        label_text = f"{label}: {prob.item():.2f}"
        cv2.putText(frame, label_text, (x1, y1 - 10), cv2.FONT_HERSHEY_SIMPLEX, 0.9, (0, 255, 0), 2)

    cv2.imshow("YOLOv8 Object Detection", frame)

    if cv2.waitKey(1) & 0xFF == ord('q'):
        break
cap.release()
cv2.destroyAllWindows()