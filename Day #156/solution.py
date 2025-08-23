# Day 156: Min perfect squares summing to n via DP. dp[i] = min over squares
# j*j<=i of dp[i-j*j]+1. Time O(n*sqrt(n)), Space O(n).


def num_squares(n):
    dp = [0] + [float("inf")] * n
    for i in range(1, n + 1):
        j = 1
        while j * j <= i:
            dp[i] = min(dp[i], dp[i - j * j] + 1)
            j += 1
    return dp[n]


if __name__ == "__main__":
    print(num_squares(13))  # 2
    print(num_squares(27))  # 3
