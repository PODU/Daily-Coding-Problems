# Day 1427: Rotate array right by k in-place.
# Approach: triple reversal (reverse all, reverse first k, reverse rest).
# Time: O(n), Space: O(1).

def rotate(a, k):
    n = len(a)
    if n == 0:
        return
    k %= n

    def rev(i, j):
        while i < j:
            a[i], a[j] = a[j], a[i]
            i += 1
            j -= 1

    rev(0, n - 1)
    rev(0, k - 1)
    rev(k, n - 1)


if __name__ == "__main__":
    arr = [1, 2, 3, 4, 5, 6, 7]
    rotate(arr, 3)
    print(arr)  # [5, 6, 7, 1, 2, 3, 4]
