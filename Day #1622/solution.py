# Day 1622: Longest substring with at most k distinct characters.
# Sliding window with a count map. Time O(n), Space O(k).
def longest_k_distinct(s, k):
    if k <= 0:
        return ""
    cnt = {}
    left = best_l = best_len = 0
    for right, c in enumerate(s):
        cnt[c] = cnt.get(c, 0) + 1
        while len(cnt) > k:
            lc = s[left]
            cnt[lc] -= 1
            if cnt[lc] == 0:
                del cnt[lc]
            left += 1
        if right - left + 1 > best_len:
            best_len = right - left + 1
            best_l = left
    return s[best_l:best_l + best_len]


if __name__ == "__main__":
    print('"' + longest_k_distinct("abcba", 2) + '"')
