# Day 807: Longest substring with at most k distinct characters.
# Sliding window + char count map; shrink left when distinct > k. Time O(N), Space O(k).
from collections import defaultdict


def longest_k_distinct(s, k):
    if k == 0:
        return 0
    cnt = defaultdict(int)
    left = best = 0
    for right, c in enumerate(s):
        cnt[c] += 1
        while len(cnt) > k:
            cnt[s[left]] -= 1
            if cnt[s[left]] == 0:
                del cnt[s[left]]
            left += 1
        best = max(best, right - left + 1)
    return best


if __name__ == "__main__":
    print(longest_k_distinct("abcba", 2))  # 3
