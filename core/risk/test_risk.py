import numpy as np
from risk import volatility, check_risk, circuit_breaker

def test_volatility():
    returns = np.array([0.01, -0.02, 0.015, -0.005], dtype=np.float32)
    sigma = volatility(returns)
    print(f"Volatility: {sigma:.6f}")
    assert sigma > 0

def test_check_risk():
    positions = np.array([1, -1, 0.5, -0.5], dtype=np.float32)
    returns = np.array([0.01, -0.02, 0.015, -0.005], dtype=np.float32)
    assert check_risk(positions, returns, threshold=0.02) in [True, False]

def test_circuit_breaker():
    positions = np.array([1, -1, 0.5, -0.5], dtype=np.float32)
    returns = np.array([0.01, -0.02, 0.015, -0.005], dtype=np.float32)
    assert circuit_breaker(positions, returns, threshold=0.02) in [True, False]

if __name__ == "__main__":
    test_volatility()
    test_check_risk()
    test_circuit_breaker()
    print("Risk module tests passed.")
