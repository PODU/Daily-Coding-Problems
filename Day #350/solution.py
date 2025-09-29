# Day 350: Min perfect squares summing to N via DP, then greedy-largest reconstruction.
# Time O(N*sqrt N), Space O(N).
import math


def solve(n):
    dp = [float("inf")] * (n + 1)
    dp[0] = 0
    for i in range(1, n + 1):
        s = 1
        while s * s <= i:
            dp[i] = min(dp[i], dp[i - s * s] + 1)
            s += 1

    # Reconstruct: greedily pick largest s with dp[i - s*s] == dp[i] - 1.
    squares = []
    i = n
    while i > 0:
        for s in range(int(math.isqrt(i)), 0, -1):
            if dp[i - s * s] == dp[i] - 1:
                squares.append(s * s)
                i -= s * s
                break

    return f"{dp[n]} (" + " + ".join(str(x) for x in squares) + ")"


if __name__ == "__main__":
    for n in (4, 17, 18):
        print(solve(n))
