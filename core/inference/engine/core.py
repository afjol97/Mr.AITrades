import torch
from typing import Optional

class InferenceEngine:
    def __init__(self, model_path: str):
        # Load model and warm up
        self.model = torch.jit.load(model_path).cuda()
        self.model.eval()
        
        # FP16 conversion (no torch-tensorrt)
        self.model = self.model.half()
        
        # CUDA Graph setup
        self.static_input = torch.randn(1, 64, dtype=torch.float16).cuda()
        self.static_output = torch.empty_like(self.static_input)
        self.graph = torch.cuda.CUDAGraph()
        self._capture_graph()

    def _capture_graph(self):
        """Capture inference as a CUDA graph for zero-launch overhead"""
        with torch.cuda.stream(torch.cuda.Stream()):
            self.graph.capture_begin()
            self.static_output = self.model(self.static_input)
            self.graph.capture_end()

    def infer(self, input: torch.Tensor) -> torch.Tensor:
        """Execute inference with 0 Python overhead"""
        assert input.shape == self.static_input.shape
        self.static_input.copy_(input)
        self.graph.replay()
        return self.static_output.clone()

    @staticmethod
    def validate_latency(engine, test_input: torch.Tensor, n_runs: int = 1000) -> float:
        """Benchmark average inference latency in microseconds"""
        start_event = torch.cuda.Event(enable_timing=True)
        end_event = torch.cuda.Event(enable_timing=True)
        
        torch.cuda.synchronize()
        start_event.record()
        for _ in range(n_runs):
            _ = engine.infer(test_input)
        end_event.record()
        torch.cuda.synchronize()
        
        return start_event.elapsed_time(end_event) * 1000 / n_runs  # µs
