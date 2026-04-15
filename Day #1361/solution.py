# Day 1361: Max profit, unlimited transactions with a fixed fee per completed sale.
# DP states cash/hold tracked greedily. Time O(n), Space O(1).


def max_profit(prices, fee):
    cash, hold = 0, float("-inf")
    for p in prices:
        hold = max(hold, cash - p)
        cash = max(cash, hold + p - fee)
    return cash


if __name__ == "__main__":
    prices = [1, 3, 2, 8, 4, 10]
    fee = 2
    print(max_profit(prices, fee))
