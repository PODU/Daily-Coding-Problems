# Day 13: Longest substring with at most k distinct chars: sliding window + count map.
# Time: O(n), Space: O(k).
def longest_k_distinct(s, k):
    if k == 0:
        return 0
    cnt = {}
    left = best = 0
    for right, c in enumerate(s):
        cnt[c] = cnt.get(c, 0) + 1
        while len(cnt) > k:
            lc = s[left]
            cnt[lc] -= 1
            if cnt[lc] == 0:
                del cnt[lc]
            left += 1
        best = max(best, right - left + 1)
    return best


if __name__ == "__main__":
    print(longest_k_distinct("abcba", 2))  # 3
