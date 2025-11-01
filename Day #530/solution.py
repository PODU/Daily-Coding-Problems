# Day 530: Levenshtein edit distance via DP with rolling array.
# Time O(m*n), Space O(min(m,n)).


def edit_distance(a: str, b: str) -> int:
    if len(a) < len(b):
        a, b = b, a
    n = len(b)
    prev = list(range(n + 1))
    for i in range(1, len(a) + 1):
        cur = [i] + [0] * n
        for j in range(1, n + 1):
            cost = 0 if a[i - 1] == b[j - 1] else 1
            cur[j] = min(prev[j] + 1, cur[j - 1] + 1, prev[j - 1] + cost)
        prev = cur
    return prev[n]


if __name__ == "__main__":
    print(edit_distance("kitten", "sitting"))
