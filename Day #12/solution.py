# Day 12: Staircase ways with step set X: dp[i] = sum dp[i-x] for x in X.
# Time: O(N*|X|), Space: O(N).
def staircase(n, X=(1, 2)):
    dp = [0] * (n + 1)
    dp[0] = 1
    for i in range(1, n + 1):
        for x in X:
            if i - x >= 0:
                dp[i] += dp[i - x]
    return dp[n]


if __name__ == "__main__":
    print(staircase(4, (1, 2)))  # 5
