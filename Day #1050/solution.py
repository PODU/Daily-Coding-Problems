# Day 1050: Apply permutation where result[P[i]] = array[i] (scatter). In-place cycle-following via swaps:
# O(n) time, O(1) extra space. Also a simple O(n)-space scatter is provided.


def apply_in_place(a, P):
    # Follow each cycle with swaps; each swap settles an element, so <= n swaps total.
    P = P[:]  # local copy so caller's permutation is untouched
    n = len(a)
    for i in range(n):
        while P[i] != i:
            j = P[i]
            a[i], a[j] = a[j], a[i]
            P[i], P[j] = P[j], P[i]
    return a


def apply_simple(a, P):
    res = [None] * len(a)
    for i in range(len(a)):
        res[P[i]] = a[i]
    return res


if __name__ == "__main__":
    a = ["a", "b", "c"]
    P = [2, 1, 0]  # result[P[i]] = a[i]
    apply_in_place(a, P)
    print("[" + ", ".join(a) + "]")
