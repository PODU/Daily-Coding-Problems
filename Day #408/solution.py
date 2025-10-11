# Day 408: Max profit with at most k stock transactions.
# Approach: DP tracking best buy/sell state per transaction in one pass.
# Time: O(n*k), Space: O(k). Example k=2, [5,2,4,0,1] -> 3.


def max_profit(k, prices):
    n = len(prices)
    if n == 0 or k == 0:
        return 0
    # If k >= n//2, unlimited transactions are possible.
    if k >= n // 2:
        return sum(max(prices[i] - prices[i - 1], 0) for i in range(1, n))
    buy = [float("-inf")] * (k + 1)
    sell = [0] * (k + 1)
    for price in prices:
        for t in range(1, k + 1):
            buy[t] = max(buy[t], sell[t - 1] - price)
            sell[t] = max(sell[t], buy[t] + price)
    return sell[k]


if __name__ == "__main__":
    print(max_profit(2, [5, 2, 4, 0, 1]))  # 3
