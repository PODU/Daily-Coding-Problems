# Day 972: Rearrange string so no two adjacent chars match (else None).
# Approach: place most frequent chars into even-then-odd slots. Time O(n log k), Space O(n).
from collections import Counter


def rearrange(s):
    n = len(s)
    cnt = Counter(s)
    order = sorted(cnt.items(), key=lambda kv: -kv[1])
    if order[0][1] > (n + 1) // 2:
        return None
    res = [''] * n
    idx = 0
    for ch, c in order:
        for _ in range(c):
            res[idx] = ch
            idx += 2
            if idx >= n:
                idx = 1
    return ''.join(res)


if __name__ == '__main__':
    print(rearrange("aaabbc"))  # ababac
    print(rearrange("aaab"))    # None
