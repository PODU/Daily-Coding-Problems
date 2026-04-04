# Day 1299: Count ordered ways to climb N stairs using step sizes from set X.
# DP: dp[i] = sum(dp[i-x]) for x in X. O(N*|X|) time, O(N) space.


def climb_ways(n: int, steps=(1, 2)) -> int:
    dp = [0] * (n + 1)
    dp[0] = 1
    for i in range(1, n + 1):
        for x in steps:
            if i - x >= 0:
                dp[i] += dp[i - x]
    return dp[n]


if __name__ == "__main__":
    print(climb_ways(4, (1, 2)))     # 5
    print(climb_ways(10, (1, 3, 5)))  # generalized
