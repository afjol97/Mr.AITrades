# strategy.py

def run_strategy(quote):
    """
    Example trading logic: returns a signal dict if a trade should be made, else None.
    Modify this function to implement your own strategy.
    """
    # Example: simple threshold logic for BTCUSDT
    if quote.get('symbol') == 'BTCUSDT' and quote.get('bid', 0) > 69000:
        return {'action': 'SELL', 'symbol': quote['symbol'], 'price': quote['bid']}
    # Example: buy ETHUSDT if ask < 3500
    if quote.get('symbol') == 'ETHUSDT' and quote.get('ask', 0) < 3500:
        return {'action': 'BUY', 'symbol': quote['symbol'], 'price': quote['ask']}
    return None

def on_quote(quote):
    print(f"📈 Got quote: {quote}")
    signal = run_strategy(quote)
    if signal:
        print(f"🚨 Signal: {signal}")
