# Day 1408: Pancake sort: for each shrinking prefix find its max, flip it to the front,
# then flip it into its final position. Only uses reverse(lst, i, j).
# Time O(n^2) comparisons, O(n) flips, Space O(1).

def reverse(lst, i, j):
    while i < j:
        lst[i], lst[j] = lst[j], lst[i]
        i += 1
        j -= 1


def pancake_sort(a):
    n = len(a)
    while n > 1:
        mi = max(range(n), key=lambda i: a[i])
        if mi != n - 1:
            reverse(a, 0, mi)
            reverse(a, 0, n - 1)
        n -= 1
    return a


if __name__ == "__main__":
    print(pancake_sort([3, 1, 4, 1, 5, 9, 2, 6]))
