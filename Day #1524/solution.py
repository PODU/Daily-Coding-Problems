# Day 1524: First missing positive: cyclic sort (index-as-hash), place each x in [1,n] at index x-1.
# Time O(n), Space O(1) extra (in-place).
def first_missing_positive(a):
    a = list(a)
    n = len(a)
    for i in range(n):
        while 1 <= a[i] <= n and a[a[i] - 1] != a[i]:
            j = a[i] - 1
            a[i], a[j] = a[j], a[i]
    for i in range(n):
        if a[i] != i + 1:
            return i + 1
    return n + 1


if __name__ == "__main__":
    print(first_missing_positive([3, 4, -1, 1]))  # 2
    print(first_missing_positive([1, 2, 0]))      # 3
