# Day 572: Next greater permutation in-place (lexicographic). If none, wrap to smallest.
# Approach: find pivot, successor swap, reverse suffix. Time O(n), Space O(1).


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


def run(a):
    print("[" + ",".join(map(str, next_permutation(a))) + "]")


if __name__ == "__main__":
    run([1, 2, 3])
    run([1, 3, 2])
    run([3, 2, 1])
