import time
import threading
from queue import Queue, Empty

class Order:
    def __init__(self, symbol, side, qty, px=None, ts=None, signal=None):
        self.symbol = symbol
        self.side = side  # 'buy' or 'sell'
        self.qty = qty
        self.px = px
        self.ts = ts or time.time()
        self.signal = signal
        self.status = 'pending'  # 'pending', 'filled', 'cancelled', 'error'
        self.error = None
        self.exchange_order_id = None

class OrderExecutor:
    def __init__(self, exchange_adapter, symbol='BTCUSDT'):
        self.exchange = exchange_adapter
        self.symbol = symbol
        self.orders = []
        self.order_queue = Queue()
        self._running = False
        self._thread = None

    def submit_signal(self, signal):
        # Map signal to order
        if signal.kind == 'buy':
            order = Order(self.symbol, 'buy', qty=1.0, signal=signal)
            self.order_queue.put(order)
        elif signal.kind == 'sell':
            order = Order(self.symbol, 'sell', qty=1.0, signal=signal)
            self.order_queue.put(order)
        # 'hold' or others: do nothing

    def start(self):
        self._running = True
        self._thread = threading.Thread(target=self._run_loop, daemon=True)
        self._thread.start()

    def stop(self):
        self._running = False
        if self._thread:
            self._thread.join()

    def _run_loop(self):
        while self._running:
            try:
                order = self.order_queue.get(timeout=0.1)
                self._execute_order(order)
            except Empty:
                continue

    def _execute_order(self, order, max_retries=3):
        print(f"[DEBUG] Executing order: {order.symbol} {order.side} qty={order.qty} (status={order.status})")
        order.status = 'pending'  # Mark as pending before execution
        for attempt in range(max_retries):
            try:
                # Place order via exchange adapter (stubbed for now)
                order.exchange_order_id = self.exchange.place_order(
                    symbol=order.symbol, side=order.side, qty=order.qty, px=order.px
                )
                order.status = 'filled'
                print(f"[DEBUG] Order filled: id={order.exchange_order_id}")
                self.orders.append(order)
                return
            except Exception as e:
                order.error = str(e)
                print(f"[DEBUG] Order execution error: {order.error}")
                time.sleep(0.05)
        order.status = 'error'
        print(f"[DEBUG] Order failed after retries: {order.symbol} {order.side}")
        self.orders.append(order)

    def cancel_all(self):
        # Cancel all open orders (stub)
        for order in self.orders:
            if order.status == 'pending':
                try:
                    self.exchange.cancel_order(order.exchange_order_id)
                    order.status = 'cancelled'
                except Exception as e:
                    order.error = str(e)

    def get_filled_orders(self):
        # Return orders that are filled or were filled after execution
        return [o for o in self.orders if getattr(o, 'status', None) == 'filled']

    def get_errors(self):
        # Return orders that errored after execution
        return [o for o in self.orders if getattr(o, 'status', None) == 'error']

    def process_all(self):
        """Synchronously process all orders in the queue (for testing)."""
        print(f"[DEBUG] Processing all orders in queue (size={self.order_queue.qsize()})")
        while not self.order_queue.empty():
            try:
                order = self.order_queue.get_nowait()
                print(f"[DEBUG] Got order from queue: {order.symbol} {order.side} (status={order.status})")
                self._execute_order(order)
            except Exception as e:
                print(f"[DEBUG] Exception in process_all: {e}")
                break
