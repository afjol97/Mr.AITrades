import numpy as np
import pandas as pd
import time
import threading
from queue import Queue

class Signal:
    def __init__(self, kind, ts, confidence=1.0, meta=None):
        self.kind = kind  # 'buy', 'sell', 'hold'
        self.ts = ts
        self.confidence = confidence
        self.meta = meta or {}

    def to_dict(self):
        return {
            'kind': self.kind,
            'ts': self.ts,
            'confidence': self.confidence,
            **self.meta
        }

class SignalEngine:
    def __init__(self, window_sizes=(1, 5, 60)):
        self.ticks = []
        self.order_books = []
        self.window_sizes = window_sizes  # seconds
        self.state = {
            'best_bid': None,
            'best_ask': None,
            'microprice': None,
            'midprice': None,
            'last_trade': None,
            'imbalance': None,
        }
        self.signal_queue = Queue()
        self.signal_log = []
        self._lock = threading.Lock()

    def on_tick(self, tick):
        with self._lock:
            self.ticks.append(tick)
            self.state['last_trade'] = tick
            self._update_windows()
            # Always emit a signal, even if order book is missing
            bid = self.state['best_bid']
            ask = self.state['best_ask']
            ts = time.time()
            if bid is not None and ask is not None:
                if bid > ask:
                    self._emit_signal('buy', ts, 1.0)
                else:
                    self._emit_signal('hold', ts, 0.5)
            else:
                self._emit_signal('hold', ts, 0.1)  # No book context: emit Hold/No Action

    def on_order_book(self, ob):
        with self._lock:
            self.order_books.append(ob)
            self._update_state_from_book(ob)
            self._update_windows()
            self._maybe_emit_signal()

    def ingest_order_book_snapshot(self, ob):
        """Ingest a full order book snapshot and update state."""
        with self._lock:
            self.order_books.append(ob)
            self._update_state_from_book(ob)
            self._update_windows()

    def _update_state_from_book(self, ob):
        bids = ob.get('bids', [])
        asks = ob.get('asks', [])
        if bids:
            self.state['best_bid'] = max(bids, key=lambda x: x[0])[0]
        if asks:
            self.state['best_ask'] = min(asks, key=lambda x: x[0])[0]
        if bids and asks:
            self.state['midprice'] = (self.state['best_bid'] + self.state['best_ask']) / 2
            self.state['microprice'] = (
                self.state['best_bid'] * asks[0][1] + self.state['best_ask'] * bids[0][1]
            ) / (asks[0][1] + bids[0][1])
            total_bid = sum(q for _, q in bids)
            total_ask = sum(q for _, q in asks)
            self.state['imbalance'] = (total_bid - total_ask) / (total_bid + total_ask + 1e-9)

    def _update_windows(self):
        now = time.time()
        for arr in [self.ticks, self.order_books]:
            while arr and now - arr[0]['ts'] > max(self.window_sizes):
                arr.pop(0)

    def _maybe_emit_signal(self):
        # Dummy strategy: if best_bid > best_ask, emit Buy
        bid = self.state['best_bid']
        ask = self.state['best_ask']
        ts = time.time()
        if bid is not None and ask is not None:
            if bid > ask:
                self._emit_signal('buy', ts, 1.0)
            else:
                self._emit_signal('hold', ts, 0.5)

    def _emit_signal(self, kind, ts, confidence):
        sig = Signal(kind, ts, confidence, meta=self.state.copy())
        self.signal_queue.put(sig)
        self.signal_log.append(sig.to_dict())

    def get_signal(self):
        try:
            return self.signal_queue.get_nowait()
        except Exception:
            return None

    def save_log(self, path='signal_log.csv'):
        pd.DataFrame(self.signal_log).to_csv(path, index=False)

    def latency_benchmark(self, n=10000):
        start = time.perf_counter()
        for _ in range(n):
            self.on_tick({'ts': time.time(), 'px': 1.0, 'qty': 1.0})
        end = time.perf_counter()
        return (end - start) / n * 1e6  # μs per event
