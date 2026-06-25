# Day 1715: DP states cash/hold; fee charged once per buy-sell on sell. Time O(n), Space O(1).
def max_profit(prices, fee):
    if not prices:
        return 0
    cash, hold = 0, -prices[0]
    for p in prices[1:]:
        cash = max(cash, hold + p - fee)
        hold = max(hold, cash - p)
    return cash


if __name__ == "__main__":
    prices = [1, 3, 2, 8, 4, 10]
    fee = 2
    print(max_profit(prices, fee))
