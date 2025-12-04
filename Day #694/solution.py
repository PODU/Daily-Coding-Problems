# Day 694: First missing positive integer in O(n) time, O(1) space.
# Approach: cyclic sort - place each value v in [1,n] at index v-1, then scan.
# Time O(n), Space O(1) (in-place).


def first_missing_positive(a):
    n = len(a)
    for i in range(n):
        while 0 < a[i] <= n and a[a[i] - 1] != a[i]:
            j = a[i] - 1
            a[i], a[j] = a[j], a[i]
    for i in range(n):
        if a[i] != i + 1:
            return i + 1
    return n + 1


if __name__ == "__main__":
    print(first_missing_positive([3, 4, -1, 1]))  # 2
    print(first_missing_positive([1, 2, 0]))      # 3
