# Day 883: Min steps to 1: dp[i] = 1 + min(dp[i-1], dp[i/a] over factors a). Time O(N*sqrt N), Space O(N).

def min_steps(n):
    dp = [0] * (n + 1)
    for i in range(2, n + 1):
        dp[i] = dp[i - 1] + 1
        a = 2
        while a * a <= i:
            if i % a == 0:
                larger = i // a
                dp[i] = min(dp[i], 1 + dp[larger])
            a += 1
    return dp[n]


if __name__ == "__main__":
    print(min_steps(100))
