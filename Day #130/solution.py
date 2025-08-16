# Day 130: Max profit with at most k buy/sell transactions.
# DP with hold/cash per transaction. O(n*k) time, O(k) space (greedy when k large).


def max_profit(k, p):
    n = len(p)
    if n == 0 or k == 0:
        return 0
    if k >= n // 2:
        return sum(max(0, p[i] - p[i - 1]) for i in range(1, n))
    buy = [float("-inf")] * (k + 1)
    sell = [0] * (k + 1)
    for price in p:
        for j in range(1, k + 1):
            buy[j] = max(buy[j], sell[j - 1] - price)
            sell[j] = max(sell[j], buy[j] + price)
    return sell[k]


if __name__ == "__main__":
    prices = [5, 2, 4, 0, 1]
    print(max_profit(2, prices))  # 3
