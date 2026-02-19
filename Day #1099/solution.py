# Day 1099: Rotate array right by k in-place using the reversal algorithm.
# Reverse whole, reverse first k, reverse rest. Time: O(N). Space: O(1).
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
    a = [1, 2, 3, 4, 5, 6, 7]
    rotate(a, 3)
    print(a)  # [5, 6, 7, 1, 2, 3, 4]
