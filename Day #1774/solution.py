# Day 1774: Fewest steps to reduce N to 1 (decrement by 1, or jump to the larger
# factor of any factorization). DP over 1..N, trying every divisor. O(N*sqrt N) time, O(N) space.


def min_steps(N):
    dp = [0] * (N + 1)
    for i in range(2, N + 1):
        dp[i] = dp[i - 1] + 1            # decrement step
        a = 2
        while a * a <= i:
            if i % a == 0:
                dp[i] = min(dp[i], dp[i // a] + 1)  # jump to larger factor i//a
            a += 1
    return dp[N]


if __name__ == "__main__":
    print(min_steps(100))  # 100->10->9->3->2->1 = 5
