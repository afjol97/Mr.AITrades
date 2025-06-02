import torch
from engine import InferenceEngine

def test_engine():
    # Mock model (replace with actual trained model)
    model = torch.nn.Linear(64, 64).cuda()
    traced_model = torch.jit.trace(model, torch.randn(1, 64).cuda())
    torch.jit.save(traced_model, "test_model.pt")
    
    # Initialize engine
    engine = InferenceEngine("test_model.pt")
    test_input = torch.randn(1, 64, dtype=torch.float16).cuda()
    
    # Warmup and benchmark
    latency = InferenceEngine.validate_latency(engine, test_input)
    print(f"Average Inference Latency: {latency:.2f}µs")
    assert latency < 100, "Latency exceeds 100µs target!"

if __name__ == "__main__":
    test_engine()
