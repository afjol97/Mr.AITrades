import sys
import os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from signal_engine import SignalEngine
import pytest
import time

def test_signal_engine_tick():
    engine = SignalEngine()
    tick = {'ts': time.time(), 'px': 100.0, 'qty': 1.0}
    engine.on_tick(tick)
    sig = engine.get_signal()
    assert sig is not None
    assert sig.kind in ('buy', 'hold')

def test_signal_engine_order_book():
    engine = SignalEngine()
    ob = {'ts': time.time(), 'bids': [(100.0, 1.0)], 'asks': [(101.0, 1.0)]}
    engine.on_order_book(ob)
    sig = engine.get_signal()
    assert sig is not None
    assert sig.kind in ('buy', 'hold')

def test_latency_benchmark():
    engine = SignalEngine()
    latency = engine.latency_benchmark(1000)
    assert latency < 500  # μs per event (relaxed for CI)

def test_tick_then_order_book():
    engine = SignalEngine()
    tick = {'ts': time.time(), 'px': 100.0, 'qty': 1.0}
    ob = {'ts': time.time(), 'bids': [(100.0, 1.0)], 'asks': [(101.0, 1.0)]}
    engine.on_tick(tick)
    sig1 = engine.get_signal()
    assert sig1 is not None
    assert sig1.kind == 'hold'  # No book context
    engine.ingest_order_book_snapshot(ob)
    engine.on_tick({'ts': time.time(), 'px': 101.0, 'qty': 2.0})
    sig2 = engine.get_signal()
    assert sig2 is not None
    assert sig2.kind in ('buy', 'hold')

def test_signal_to_executor():
    from order_executor_stub import OrderExecutorStub
    engine = SignalEngine()
    ob = {'ts': time.time(), 'bids': [(100.0, 1.0)], 'asks': [(101.0, 1.0)]}
    engine.ingest_order_book_snapshot(ob)
    engine.on_tick({'ts': time.time(), 'px': 101.0, 'qty': 2.0})
    executor = OrderExecutorStub()
    executor.consume_signals(engine)
    signals = executor.get_all()
    assert signals
    assert signals[0].kind in ('buy', 'hold')

def test_order_execution_flow():
    from exchange_adapter_stub import ExchangeAdapterStub
    from order_executor import OrderExecutor
    from signal_engine import SignalEngine
    import time
    adapter = ExchangeAdapterStub()
    executor = OrderExecutor(adapter)
    engine = SignalEngine()
    ob = {'ts': time.time(), 'bids': [(100.0, 1.0)], 'asks': [(101.0, 1.0)]}
    engine.ingest_order_book_snapshot(ob)
    engine.on_tick({'ts': time.time(), 'px': 101.0, 'qty': 2.0})
    sig = engine.get_signal()
    executor.submit_signal(sig)
    executor.process_all()  # Synchronous processing for test
    filled = executor.get_filled_orders()
    assert filled
    assert filled[0].exchange_order_id.startswith('EX-')

def test_order_error_and_cancel():
    class FailingAdapter:
        def place_order(self, *a, **kw):
            raise Exception('fail')
        def cancel_order(self, oid):
            return True
    from order_executor import OrderExecutor
    executor = OrderExecutor(FailingAdapter())
    from signal_engine import SignalEngine
    engine = SignalEngine()
    ob = {'ts': time.time(), 'bids': [(100.0, 1.0)], 'asks': [(101.0, 1.0)]}
    engine.ingest_order_book_snapshot(ob)
    engine.on_tick({'ts': time.time(), 'px': 101.0, 'qty': 2.0})
    sig = engine.get_signal()
    executor.submit_signal(sig)
    executor.process_all()  # Synchronous processing for test
    errors = executor.get_errors()
    assert errors
    executor.cancel_all()
