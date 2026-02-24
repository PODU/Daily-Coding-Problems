# Day 1121: Day 1121 - Max stock profit with a transaction fee, unlimited transactions
# Approach: state machine DP tracking best cash (no holding) and best hold.
# Time: O(n), Space: O(1).

def max_profit(prices, fee):
    cash, hold = 0, -prices[0]
    for p in prices[1:]:
        cash = max(cash, hold + p - fee)
        hold = max(hold, cash - p)
    return cash


if __name__ == "__main__":
    print(max_profit([1, 3, 2, 8, 4, 10], 2))  # 9
