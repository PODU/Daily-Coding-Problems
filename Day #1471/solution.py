# Day 1471: Search a sorted list in O(log N) without multiply/divide/bit-shift.
# Precompute powers of two by doubling (addition only), then do jump-based
# binary search. Time O(log N), Space O(log N).

def search(a, x):
    n = len(a)
    if n == 0:
        return False
    powers = []
    p = 1
    while p <= n:
        powers.append(p)
        p = p + p  # doubling via addition only (no *, /, or shift)
    pos = -1
    for p in reversed(powers):
        nxt = pos + p
        if nxt < n and a[nxt] <= x:
            pos = nxt
    return pos >= 0 and a[pos] == x


if __name__ == "__main__":
    arr = [1, 3, 5, 7, 9, 11]
    print(search(arr, 7))  # True
    print(search(arr, 8))  # False
