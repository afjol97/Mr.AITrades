import sys
import os
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '../core/inference')))

import time
import subprocess
import numpy as np
from ffi import SharedTensor
from runner import execute_inference

def test_pipeline(iterations=10, log_path="e2e_latency_audit.log"):
    latencies = []
    rust_process = None
    try:
        # 1. Start Rust data feed
        rust_process = subprocess.Popen(["../../target/release/hft_bot"])
        time.sleep(0.1)  # Give Rust process time to initialize

        # 2. Simulate market data
        tensor = SharedTensor("/hft_bot_features", 64 * 4)
        with open(log_path, "w") as logf:
            logf.write("# E2E Latency Audit Log\n")
            for i in range(iterations):
                test_data = np.random.randn(64).astype(np.float32)
                start = time.perf_counter_ns()
                tensor.write(test_data)  # Rust -> Python
                try:
                    signal = execute_inference()  # Inference
                except Exception as e:
                    logf.write(f"Iteration {i}: Inference error: {e}\n")
                    continue
                if signal is None:
                    logf.write(f"Iteration {i}: No signal returned\n")
                    continue
                latency = (time.perf_counter_ns() - start) / 1000  # µs
                latencies.append(latency)
                logf.write(f"Iteration {i}: {latency:.2f}µs\n")
                print(f"Iteration {i}: E2E Latency: {latency:.2f}µs")
                if latency >= 10:
                    logf.write(f"Iteration {i}: FAIL - Exceeds 10µs target!\n")
            if latencies:
                logf.write(f"\nSummary: min={min(latencies):.2f}µs, avg={np.mean(latencies):.2f}µs, max={max(latencies):.2f}µs\n")
                print(f"\nSummary: min={min(latencies):.2f}µs, avg={np.mean(latencies):.2f}µs, max={max(latencies):.2f}µs")
            else:
                logf.write("No successful latency measurements.\n")
                print("No successful latency measurements.")
    except Exception as e:
        print(f"Test pipeline error: {e}")
    finally:
        if rust_process is not None:
            rust_process.terminate()
            try:
                rust_process.wait(timeout=2)
            except subprocess.TimeoutExpired:
                rust_process.kill()

if __name__ == "__main__":
    test_pipeline()
