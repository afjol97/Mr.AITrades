import numpy as np

def volatility(returns: np.ndarray) -> float:
    """Compute volatility (σ) as sqrt(sum(returns^2) / n)"""
    return np.sqrt(np.sum(returns ** 2) / len(returns))

def check_risk(positions: np.ndarray, returns: np.ndarray, threshold: float = 0.02) -> bool:
    """Return True if risk is acceptable (max loss < threshold), else False."""
    vol = volatility(returns)
    max_loss = np.sum(positions * vol)
    return max_loss < threshold

def circuit_breaker(positions: np.ndarray, returns: np.ndarray, threshold: float = 0.02) -> bool:
    """Pause trading if drawdown exceeds threshold."""
    return not check_risk(positions, returns, threshold)
