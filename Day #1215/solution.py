# Day 1215: Min steps to reduce N to 1 (decrement, or replace by larger factor).
# DP: dp[i] = 1 + min(dp[i-1], dp[i//d] for divisors d). Time O(N sqrt N), Space O(N).


def min_steps(n):
    dp = [0] * (n + 1)
    for i in range(2, n + 1):
        dp[i] = dp[i - 1] + 1  # decrement
        d = 2
        while d * d <= i:
            if i % d == 0:
                dp[i] = min(dp[i], dp[i // d] + 1)  # larger factor i//d
            d += 1
    return dp[n]


if __name__ == "__main__":
    print(min_steps(100))  # 5 (100 -> 10 -> 9 -> 3 -> 2 -> 1)
