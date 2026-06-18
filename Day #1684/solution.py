# Day 1684: Levenshtein edit distance via 1D rolling DP.
# Time O(n*m), Space O(min(n,m)).


def edit_distance(a, b):
    n, m = len(a), len(b)
    dp = list(range(m + 1))
    for i in range(1, n + 1):
        prev = dp[0]
        dp[0] = i
        for j in range(1, m + 1):
            tmp = dp[j]
            if a[i - 1] == b[j - 1]:
                dp[j] = prev
            else:
                dp[j] = 1 + min(prev, dp[j], dp[j - 1])
            prev = tmp
    return dp[m]


if __name__ == "__main__":
    print(edit_distance("kitten", "sitting"))  # 3
