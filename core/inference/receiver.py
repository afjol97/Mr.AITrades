import numpy as np
import pandas as pd
from exchange_adapter import ffi_ring_pop, ffi_ring_len
import time

class MarketDataReceiver:
    def __init__(self):
        self.last_ts = None
        self.latencies = []

    def poll(self, max_msgs=1000):
        msgs = []
        for _ in range(max_msgs):
            msg = ffi_ring_pop()
            if msg is None:
                break
            msgs.append(msg)
            self._record_latency(msg)
        return msgs

    def _record_latency(self, msg):
        # Assume FlatBuffer message has a u64 timestamp at offset 0 (for demo)
        ts = int.from_bytes(msg[:8], 'little')
        now = int(time.time() * 1e3)
        latency = now - (ts // 1_000_000)
        self.latencies.append(latency)
        self.last_ts = ts

    def latency_stats(self):
        arr = np.array(self.latencies)
        return {
            'count': len(arr),
            'p50': np.percentile(arr, 50) if arr.size else None,
            'p99': np.percentile(arr, 99) if arr.size else None,
            'max': arr.max() if arr.size else None,
        }

    def to_pandas(self, msgs):
        # TODO: parse FlatBuffer messages into dicts/records
        # For now, just return raw bytes
        return pd.DataFrame({'raw': msgs})
