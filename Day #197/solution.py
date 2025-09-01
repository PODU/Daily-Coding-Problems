# Day 197: Rotate array right by k in-place.
# Triple-reversal: reverse whole, reverse first k, reverse rest. O(n) time, O(1) space.


def rotate_right(a, k):
    n = len(a)
    if n == 0:
        return
    k %= n

    def reverse(i, j):
        while i < j:
            a[i], a[j] = a[j], a[i]
            i += 1
            j -= 1

    reverse(0, n - 1)
    reverse(0, k - 1)
    reverse(k, n - 1)


if __name__ == "__main__":
    a = [1, 2, 3, 4, 5]
    rotate_right(a, 2)
    print(a)  # [4, 5, 1, 2, 3]
