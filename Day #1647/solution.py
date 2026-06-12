# Day 1647: Coins-in-a-row: dp[i][j] = max value first-to-move guarantees from coins[i..j],
# dp[i][j]=max(v[i]+min(dp[i+2][j],dp[i+1][j-1]), v[j]+min(dp[i+1][j-1],dp[i][j-2])). Time/Space O(n^2).

def max_guaranteed(v):
    n = len(v)
    if n == 0:
        return 0
    dp = [[0] * n for _ in range(n)]
    for length in range(1, n + 1):
        for i in range(0, n - length + 1):
            j = i + length - 1
            a = dp[i + 2][j] if i + 2 <= j else 0
            b = dp[i + 1][j - 1] if i + 1 <= j - 1 else 0
            c = dp[i][j - 2] if i <= j - 2 else 0
            take_first = v[i] + min(a, b)
            take_last = v[j] + min(b, c)
            dp[i][j] = max(take_first, take_last)
    return dp[0][n - 1]

if __name__ == "__main__":
    coins = [3, 9, 1, 2]
    print(max_guaranteed(coins))
