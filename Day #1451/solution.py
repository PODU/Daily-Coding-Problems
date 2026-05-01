# Day 1451: Next lexicographic permutation in place (wraps to smallest).
# Find rightmost ascent, swap with next-larger suffix element, reverse suffix.
# Time O(n), Space O(1).
from typing import List


def next_permutation(a: List[int]) -> None:
    n = len(a)
    i = n - 2
    while i >= 0 and a[i] >= a[i + 1]:
        i -= 1
    if i >= 0:
        j = n - 1
        while a[j] <= a[i]:
            j -= 1
        a[i], a[j] = a[j], a[i]
    a[i + 1:] = reversed(a[i + 1:])  # i == -1 reverses whole -> smallest


def show(a):
    next_permutation(a)
    print("[" + ",".join(map(str, a)) + "]")


if __name__ == "__main__":
    show([1, 2, 3])  # [1,3,2]
    show([1, 3, 2])  # [2,1,3]
    show([3, 2, 1])  # [1,2,3]
