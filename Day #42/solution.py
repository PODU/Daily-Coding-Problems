# Day 42: Subset Sum: boolean DP dp[i][j] = can make j with first i items, then backtrack.
# Found subset is sorted descending for a deterministic [12, 9, 2, 1] output.
# Time O(n*k), Space O(n*k).

def subset_sum(S, k):
    n = len(S)
    dp = [[False] * (k + 1) for _ in range(n + 1)]
    for i in range(n + 1):
        dp[i][0] = True
    for i in range(1, n + 1):
        for j in range(1, k + 1):
            dp[i][j] = dp[i - 1][j]
            if j >= S[i - 1] and dp[i - 1][j - S[i - 1]]:
                dp[i][j] = True
    if not dp[n][k]:
        return None
    res = []
    j = k
    for i in range(n, 0, -1):
        if dp[i - 1][j]:          # item i-1 not needed
            continue
        res.append(S[i - 1])       # item i-1 must be included
        j -= S[i - 1]
    res.sort(reverse=True)
    return res


def fmt(v):
    if v is None:
        return "null"
    return "[" + ", ".join(str(x) for x in v) + "]"


if __name__ == "__main__":
    S = [12, 1, 61, 5, 9, 2]
    print(fmt(subset_sum(S, 24)))
