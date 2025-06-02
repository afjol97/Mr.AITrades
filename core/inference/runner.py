from ffi import SharedTensor
from engine import InferenceEngine

def execute_inference():
    tensor = SharedTensor("/hft_bot_features", 64 * 4)
    engine = InferenceEngine("model.pt")
    signal = engine.infer(tensor.to_tensor())
    return signal.item()  # Scalar output
