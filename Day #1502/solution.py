# Day 1502: Next greater permutation in-place. Standard next_permutation.
# Time O(n), Space O(1).

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
    return a


if __name__ == "__main__":
    for c in ([1, 2, 3], [1, 3, 2], [3, 2, 1]):
        next_permutation(c)
        print("[" + ", ".join(map(str, c)) + "]")
