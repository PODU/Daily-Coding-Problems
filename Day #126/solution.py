# Day 126: Rotate list left by k in-place via 3 reversals.
# O(n) time, O(1) extra space, ~n swaps total.


def reverse(a, i, j):
    while i < j:
        a[i], a[j] = a[j], a[i]
        i += 1
        j -= 1


def rotate_left(a, k):
    n = len(a)
    if n == 0:
        return
    k %= n
    reverse(a, 0, k - 1)
    reverse(a, k, n - 1)
    reverse(a, 0, n - 1)


if __name__ == "__main__":
    a = [1, 2, 3, 4, 5, 6]
    rotate_left(a, 2)
    print(a)
