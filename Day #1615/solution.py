# Day 1615: Rotate array right by k via three reversals: reverse all, reverse first k, reverse rest.
# Time: O(n), Space: O(1).


def rotate(a, k):
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


def main():
    a = [1, 2, 3, 4, 5, 6, 7]
    rotate(a, 3)
    print(" ".join(str(x) for x in a))


if __name__ == "__main__":
    main()
