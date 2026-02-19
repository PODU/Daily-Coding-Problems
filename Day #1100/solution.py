# Day 1100: Search sorted array in O(log N) using only addition/comparison
# (no *, /, or bit-shift). Binary lifting with powers of two built by doubling.
# Time: O(log N). Space: O(log N).
def contains(a, x):
    n = len(a)
    if n == 0:
        return False
    pows = []
    p = 1
    while p <= n:
        pows.append(p)
        p += p  # doubling via addition
    pos = -1
    for p in reversed(pows):
        if pos + p < n and a[pos + p] <= x:
            pos += p
    return pos >= 0 and a[pos] == x


if __name__ == "__main__":
    a = [1, 3, 5, 7, 9, 11]
    print(contains(a, 7))  # True
    print(contains(a, 8))  # False
