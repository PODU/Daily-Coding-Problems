# Day 1347: Reorganize string: count freqs, if max > (n+1)//2 impossible; sort chars by freq desc (tie by char),
# fill even indices first then odd. Time O(n log k), Space O(n).
from collections import Counter


def reorganize(s):
    n = len(s)
    cnt = Counter(s)
    if any(v > (n + 1) // 2 for v in cnt.values()):
        return None
    chars = sorted(cnt.keys(), key=lambda c: (-cnt[c], c))  # freq desc, tie by char asc
    res = [''] * n
    idx = 0
    for c in chars:
        for _ in range(cnt[c]):
            res[idx] = c
            idx += 2
            if idx >= n:
                idx = 1
    return ''.join(res)


if __name__ == "__main__":
    r1 = reorganize("aaabbc")
    print(r1 if r1 is not None else "None")
    r2 = reorganize("aaab")
    print(r2 if r2 is not None else "None")
