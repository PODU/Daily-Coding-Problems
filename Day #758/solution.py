# Day 758: Rotate a list left by k in place using the 3-reversal trick.
# No copy; ~n swaps total (each reversal swaps floor(len/2) pairs).
# Time: O(n), Space: O(1).


def reverse_range(a, i, j):
    swaps = 0
    while i < j:
        a[i], a[j] = a[j], a[i]
        i += 1
        j -= 1
        swaps += 1
    return swaps


def rotate_left(a, k):
    n = len(a)
    if n == 0:
        return 0
    k %= n
    swaps = 0
    swaps += reverse_range(a, 0, k - 1)
    swaps += reverse_range(a, k, n - 1)
    swaps += reverse_range(a, 0, n - 1)
    return swaps


if __name__ == "__main__":
    a = [1, 2, 3, 4, 5, 6]
    swaps = rotate_left(a, 2)
    print(a)              # [3, 4, 5, 6, 1, 2]
    print("swaps:", swaps)
