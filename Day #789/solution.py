# Day 789: Next lexicographic permutation in place (classic next_permutation). O(n) time, O(1) extra space.

def next_permutation(a):
    n = len(a)
    i = n - 2
    while i >= 0 and a[i] >= a[i + 1]:
        i -= 1
    if i >= 0:
        j = n - 1
        while a[j] <= a[i]:
            j -= 1
        a[i], a[j] = a[j], a[i]
    a[i + 1:] = reversed(a[i + 1:])


if __name__ == "__main__":
    for arr in ([1, 2, 3], [1, 3, 2], [3, 2, 1]):
        next_permutation(arr)
        print(arr)
