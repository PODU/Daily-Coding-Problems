# Day 731: Max profit from a single buy-then-sell.
# Approach: Track running minimum price and best profit in one pass.
# Time: O(n), Space: O(1).

def max_profit(prices):
    min_price, best = float("inf"), 0
    for p in prices:
        min_price = min(min_price, p)
        best = max(best, p - min_price)
    return best


if __name__ == "__main__":
    print(max_profit([9, 11, 8, 5, 7, 10]))  # 5
