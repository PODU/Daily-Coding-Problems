# Day 47: Max profit from one buy-then-sell. Track min price so far.
# Time: O(n), Space: O(1).


def max_profit(prices):
    min_price = float("inf")
    best = 0
    for p in prices:
        min_price = min(min_price, p)
        best = max(best, p - min_price)
    return best


if __name__ == "__main__":
    print(max_profit([9, 11, 8, 5, 7, 10]))
