# Day 845: rotate a list left by k in place using the 3-reversal trick.
# reverse(0,k-1), reverse(k,n-1), reverse(0,n-1). O(n) time, O(1) extra space.

def reverse(a, i, j):
    while i < j:
        a[i], a[j] = a[j], a[i]
        i += 1
        j -= 1


def rotate_left(a, k):
    n = len(a)
    if n == 0:
        return a
    k %= n
    reverse(a, 0, k - 1)
    reverse(a, k, n - 1)
    reverse(a, 0, n - 1)
    return a


if __name__ == "__main__":
    print(rotate_left([1, 2, 3, 4, 5, 6], 2))  # [3, 4, 5, 6, 1, 2]
