class ExchangeAdapterStub:
    def __init__(self):
        self.placed = []
        self.cancelled = []
        self.next_id = 1
    def place_order(self, symbol, side, qty, px=None):
        oid = f"EX-{self.next_id}"
        self.next_id += 1
        self.placed.append((oid, symbol, side, qty, px))
        return oid
    def cancel_order(self, order_id):
        self.cancelled.append(order_id)
        return True
