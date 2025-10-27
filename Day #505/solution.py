# Day 505: Rotate array right by k in-place via three reversals.
# Time O(n), Space O(1).


def reverse(a, lo, hi):
    while lo < hi:
        a[lo], a[hi] = a[hi], a[lo]
        lo += 1
        hi -= 1


def rotate_right(a, k):
    n = len(a)
    if n == 0:
        return
    k %= n
    reverse(a, 0, n - 1)
    reverse(a, 0, k - 1)
    reverse(a, k, n - 1)


def main():
    a = [1, 2, 3, 4, 5, 6, 7]
    rotate_right(a, 3)
    print(a)  # [5, 6, 7, 1, 2, 3, 4]


if __name__ == "__main__":
    main()
