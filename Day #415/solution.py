# Day 415: Max stock profit, unlimited transactions with a per-transaction fee.
# DP two states: cash (no stock) and hold (holding). Time O(N), Space O(1).
def max_profit(prices, fee):
    if not prices:
        return 0
    cash, hold = 0, -prices[0]
    for p in prices[1:]:
        cash = max(cash, hold + p - fee)
        hold = max(hold, cash - p)
    return cash


if __name__ == "__main__":
    print(max_profit([1, 3, 2, 8, 4, 10], 2))  # 9
