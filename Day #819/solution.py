# Day 819: Max profit single buy-then-sell: track min price so far and max profit in one pass. O(n) time, O(1) space.

def max_profit(prices):
    min_price = float("inf")
    best = 0
    for p in prices:
        min_price = min(min_price, p)
        best = max(best, p - min_price)
    return best


if __name__ == "__main__":
    print(max_profit([9, 11, 8, 5, 7, 10]))
