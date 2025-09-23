# Day 321: Min steps to reduce N to 1 (N-1 or replace with max factor): DP.
# Time O(N*sqrt(N)), Space O(N).
def min_steps(N):
    dp = [0] * (N + 1)
    for n in range(2, N + 1):
        dp[n] = dp[n - 1] + 1
        a = 2
        while a * a <= n:
            if n % a == 0:
                b = n // a          # b >= a, max(a,b)=b
                dp[n] = min(dp[n], dp[b] + 1)
            a += 1
    return dp[N]

if __name__ == "__main__":
    print(min_steps(100))
