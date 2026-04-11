# Day 1340: Count grid paths (right/down only) via combinatorics C(N+M-2, M-1).
# Python big ints make this exact. Time O(N+M), Space O(1).

def count_paths(n, m):
    total = n + m - 2
    k = min(n - 1, m - 1)
    res = 1
    for i in range(1, k + 1):
        res = res * (total - k + i) // i
    return res


if __name__ == "__main__":
    print(f"2 by 2 -> {count_paths(2, 2)}")
    print(f"5 by 5 -> {count_paths(5, 5)}")
