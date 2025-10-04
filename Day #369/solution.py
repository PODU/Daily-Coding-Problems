# Day 369: Stock price tracker.
# ts2price maps timestamp->price; a sorted list of prices (bisect) gives min/max
# in O(1) at the ends, running sum+count give average. add/update/remove O(log n).
from bisect import bisect_left, insort


class StockTracker:
    def __init__(self):
        self.ts2price = {}
        self.prices = []
        self.sum = 0

    def add(self, ts, price):
        self.ts2price[ts] = price
        insort(self.prices, price)
        self.sum += price

    def update(self, ts, price):
        self.remove(ts)
        self.add(ts, price)

    def remove(self, ts):
        if ts not in self.ts2price:
            return
        price = self.ts2price.pop(ts)
        self.prices.pop(bisect_left(self.prices, price))
        self.sum -= price

    def max(self):
        return self.prices[-1]

    def min(self):
        return self.prices[0]

    def average(self):
        return self.sum / len(self.prices)


if __name__ == "__main__":
    s = StockTracker()
    s.add(1, 100); s.add(2, 200); s.add(3, 150)
    print(f"max={s.max()} min={s.min()} avg={s.average():.1f}")
    s.update(2, 50)
    print(f"max={s.max()} min={s.min()} avg={s.average():.1f}")
    s.remove(3)
    print(f"max={s.max()} min={s.min()} avg={s.average():.1f}")
