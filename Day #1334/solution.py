# Day 1334: Levenshtein edit distance between two strings.
# Classic DP with rolling row. O(n*m) time, O(min(n,m)) space.

def edit_distance(a, b):
    n, m = len(a), len(b)
    prev = list(range(m + 1))
    for i in range(1, n + 1):
        cur = [i] + [0] * m
        for j in range(1, m + 1):
            if a[i - 1] == b[j - 1]:
                cur[j] = prev[j - 1]
            else:
                cur[j] = 1 + min(prev[j - 1], prev[j], cur[j - 1])
        prev = cur
    return prev[m]


if __name__ == "__main__":
    print(edit_distance("kitten", "sitting"))  # 3
