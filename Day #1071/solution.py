# Day 1071: Subset sum DP: dp[i][s] = reachable using first i items; reconstruct path. O(n*k) time/space.

def subset_sum(items, k):
    n = len(items)
    dp = [[False] * (k + 1) for _ in range(n + 1)]
    dp[0][0] = True
    for i in range(1, n + 1):
        for s in range(k + 1):
            dp[i][s] = dp[i-1][s]
            if not dp[i][s] and s >= items[i-1]:
                dp[i][s] = dp[i-1][s - items[i-1]]
    if not dp[n][k]:
        return None
    result = []
    s = k
    for i in range(n, 0, -1):
        if not dp[i-1][s]:
            result.append(items[i-1])
            s -= items[i-1]
    result.reverse()
    return result

if __name__ == "__main__":
    items = [12, 1, 61, 5, 9, 2]
    res = subset_sum(items, 24)
    print(f"Subset: {res}")
    print(f"Sum: {sum(res)}")
    print(subset_sum(items, 1000))
