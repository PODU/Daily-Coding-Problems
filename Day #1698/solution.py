# Day 1698: Deduce coin denominations from ways[] via incremental unbounded-knapsack DP.
# Time O(N^2), Space O(N).
def find_denominations(ways):
    n = len(ways)
    dp = [0] * n
    dp[0] = 1
    coins = []
    for i in range(1, n):
        if ways[i] != dp[i]:
            coins.append(i)
            for j in range(i, n):
                dp[j] += dp[j - i]
    return coins


if __name__ == "__main__":
    ways = [1, 0, 1, 1, 2]
    print(find_denominations(ways))
