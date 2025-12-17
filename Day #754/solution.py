# Day 754: Optimal coin game (interval DP / minimax).
# dp[i][j] = max value first player guarantees from coins[i..j].
# Time: O(n^2), Space: O(n^2).


def max_coins(v):
    n = len(v)
    if n == 0:
        return 0
    pre = [0] * (n + 1)
    for i in range(n):
        pre[i + 1] = pre[i] + v[i]

    dp = [[0] * n for _ in range(n)]
    for i in range(n):
        dp[i][i] = v[i]
    for length in range(2, n + 1):
        for i in range(0, n - length + 1):
            j = i + length - 1
            take_left = v[i] + (pre[j + 1] - pre[i + 1]) - dp[i + 1][j]
            take_right = v[j] + (pre[j] - pre[i]) - dp[i][j - 1]
            dp[i][j] = max(take_left, take_right)
    return dp[0][n - 1]


if __name__ == "__main__":
    coins = [8, 15, 3, 7]
    print(max_coins(coins))  # 22
