# Day 193: Max profit, unlimited transactions, fee charged per completed transaction.
# State DP: cash (no stock) / hold (holding). Time O(n), Space O(1).
from typing import List


def max_profit(prices: List[int], fee: int) -> int:
    if not prices:
        return 0
    cash, hold = 0, -prices[0]
    for p in prices[1:]:
        cash = max(cash, hold + p - fee)
        hold = max(hold, cash - p)
    return cash


if __name__ == "__main__":
    print(max_profit([1, 3, 2, 8, 4, 10], 2))
