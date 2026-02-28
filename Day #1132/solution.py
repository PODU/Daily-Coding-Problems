# Day 1132: Left-rotate list in place by k using 3 reversals. O(n) time, O(1) space.

def reverse_range(a, i, j):
    while i < j:
        a[i], a[j] = a[j], a[i]
        i += 1
        j -= 1


def rotate_left(a, k):
    n = len(a)
    if n == 0:
        return
    k %= n
    reverse_range(a, 0, k - 1)
    reverse_range(a, k, n - 1)
    reverse_range(a, 0, n - 1)


if __name__ == "__main__":
    a = [1, 2, 3, 4, 5, 6]
    rotate_left(a, 2)
    print("[" + ", ".join(str(x) for x in a) + "]")
