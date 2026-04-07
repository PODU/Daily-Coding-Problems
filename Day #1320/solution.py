# Day 1320: Longest substring with at most k distinct characters via a sliding window
# with a char-count map. Time O(n), Space O(k).
from collections import defaultdict


def longest_k_distinct(s, k):
    cnt = defaultdict(int)
    left = best_start = best_len = 0
    for right, c in enumerate(s):
        cnt[c] += 1
        while len(cnt) > k:
            cnt[s[left]] -= 1
            if cnt[s[left]] == 0:
                del cnt[s[left]]
            left += 1
        if right - left + 1 > best_len:
            best_len = right - left + 1
            best_start = left
    return s[best_start:best_start + best_len]


if __name__ == "__main__":
    sub = longest_k_distinct("abcba", 2)
    print(f'The longest substring with k distinct characters is "{sub}".')
