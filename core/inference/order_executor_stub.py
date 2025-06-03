import time
from queue import Queue

class OrderExecutorStub:
    def __init__(self):
        self.received = []
        self.queue = Queue()

    def consume_signals(self, engine, max_signals=10):
        for _ in range(max_signals):
            sig = engine.get_signal()
            if sig is not None:
                self.received.append(sig)
                self.queue.put(sig)
            else:
                break

    def get_all(self):
        out = []
        while not self.queue.empty():
            out.append(self.queue.get())
        return out
