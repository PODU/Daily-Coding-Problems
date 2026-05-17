# Day 1531: Rearrange string so no two adjacent chars equal.
# Greedy: place chars by descending frequency into even indices then odd indices.
# Time O(n + k log k), Space O(n).
from collections import Counter


def reorganize(s):
    n = len(s)
    cnt = Counter(s)
    if max(cnt.values(), default=0) > (n + 1) // 2:
        return None
    res = [''] * n
    idx = 0
    for ch, c in sorted(cnt.items(), key=lambda kv: -kv[1]):
        for _ in range(c):
            res[idx] = ch
            idx += 2
            if idx >= n:
                idx = 1
    return ''.join(res)


if __name__ == "__main__":
    r = reorganize("aaabbc")
    print(r if r is not None else "None")
