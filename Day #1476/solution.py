# Day 1476: Max profit from a single buy-then-sell.
# Track the minimum price so far and the best profit in one pass.
# Time O(N), Space O(1).

def max_profit(prices):
    min_price = float("inf")
    best = 0
    for p in prices:
        if p < min_price:
            min_price = p
        elif p - min_price > best:
            best = p - min_price
    return best


if __name__ == "__main__":
    print(max_profit([9, 11, 8, 5, 7, 10]))  # 5
