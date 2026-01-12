# Day 886: Edit distance (Levenshtein) via DP. Time O(n*m), Space O(min(n,m)).
def edit_distance(a, b):
    if len(a) > len(b):
        a, b = b, a
    n, m = len(a), len(b)
    prev = list(range(n + 1))
    for j in range(1, m + 1):
        cur = [j] + [0] * n
        for i in range(1, n + 1):
            cost = 0 if a[i - 1] == b[j - 1] else 1
            cur[i] = min(prev[i] + 1, cur[i - 1] + 1, prev[i - 1] + cost)
        prev = cur
    return prev[n]


if __name__ == "__main__":
    print(edit_distance("kitten", "sitting"))
