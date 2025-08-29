# Day 186: Minimum subset-sum difference (partition problem).
# Subset-sum DP over total, pick best <= total/2, reconstruct. Time O(n*S), Space O(n*S).
from typing import List


def solve(a: List[int]):
    n, tot = len(a), sum(a)
    dp = [[False] * (tot + 1) for _ in range(n + 1)]
    dp[0][0] = True
    for i in range(1, n + 1):
        for s in range(tot + 1):
            dp[i][s] = dp[i - 1][s] or (s >= a[i - 1] and dp[i - 1][s - a[i - 1]])
    best = 0
    for s in range(tot // 2, -1, -1):
        if dp[n][s]:
            best = s
            break
    sub, other, s = [], [], best
    for i in range(n, 0, -1):
        if s >= a[i - 1] and dp[i - 1][s - a[i - 1]]:
            sub.append(a[i - 1]); s -= a[i - 1]
        else:
            other.append(a[i - 1])
    sub.reverse(); other.reverse()
    fmt = lambda v: "{" + ", ".join(map(str, v)) + "}"
    print(f"{fmt(sub)} and {fmt(other)}")


if __name__ == "__main__":
    solve([5, 10, 15, 20, 25])
