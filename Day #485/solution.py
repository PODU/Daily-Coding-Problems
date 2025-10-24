# Day 485: Longest substring with at most k distinct characters.
# Sliding window + count map; expand right, shrink left when distinct > k. Time O(n), Space O(k).
from collections import defaultdict


def longest_k_distinct(s: str, k: int) -> int:
    if k <= 0:
        return 0
    count = defaultdict(int)
    left = 0
    best = 0
    for right, ch in enumerate(s):
        count[ch] += 1
        while len(count) > k:
            lc = s[left]
            count[lc] -= 1
            if count[lc] == 0:
                del count[lc]
            left += 1
        best = max(best, right - left + 1)
    return best


if __name__ == "__main__":
    print(longest_k_distinct("abcba", 2))  # 3
