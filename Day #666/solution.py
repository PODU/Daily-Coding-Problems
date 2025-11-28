# Day 666: Minimize difference of two subset sums. Subset-sum DP over reachable sums up to total/2,
# pick reachable sum closest to total/2, reconstruct one subset. Time O(n*sum), Space O(n*sum).


def min_diff_partition(a):
    n, total = len(a), sum(a)
    half = total // 2
    dp = [[False] * (half + 1) for _ in range(n + 1)]
    for i in range(n + 1):
        dp[i][0] = True
    for i in range(1, n + 1):
        for s in range(half + 1):
            dp[i][s] = dp[i - 1][s] or (s >= a[i - 1] and dp[i - 1][s - a[i - 1]])
    best = next(s for s in range(half, -1, -1) if dp[n][s])
    A, B, s = [], [], best
    for i in range(n, 0, -1):
        if s >= a[i - 1] and dp[i - 1][s - a[i - 1]]:
            A.append(a[i - 1])
            s -= a[i - 1]
        else:
            B.append(a[i - 1])
    return A, B, total - 2 * best


if __name__ == "__main__":
    A, B, diff = min_diff_partition([5, 10, 15, 20, 25])
    print("{%s} and {%s}, difference of %d" %
          (", ".join(map(str, A)), ", ".join(map(str, B)), diff))
