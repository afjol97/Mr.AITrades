import pytest
from exchange_adapter import ffi_ring_push, ffi_ring_pop

def test_ring_push_pop():
    data = b"test123"
    assert ffi_ring_push(data)
    out = ffi_ring_pop()
    assert out == data

def test_ring_empty():
    assert ffi_ring_pop() is None
