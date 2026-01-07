# Day 866: Max profit with at most k buy/sell transactions.
# Approach: DP with buy[j]/sell[j] rolling arrays (or greedy when k >= n/2).
# Time: O(n*k), Space: O(k).


def max_profit(k, prices):
    n = len(prices)
    if n == 0 or k == 0:
        return 0
    if k >= n // 2:
        return sum(max(0, prices[i] - prices[i - 1]) for i in range(1, n))
    buy = [float('-inf')] * (k + 1)
    sell = [0] * (k + 1)
    for p in prices:
        for j in range(1, k + 1):
            buy[j] = max(buy[j], sell[j - 1] - p)
            sell[j] = max(sell[j], buy[j] + p)
    return sell[k]


if __name__ == "__main__":
    print(max_profit(2, [5, 2, 4, 0, 1]))  # 3
