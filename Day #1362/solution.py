# Day 1362: Longest contiguous subarray with at most two distinct values.
# Sliding window + hashmap of counts, shrink when distinct > 2. Time O(n), Space O(1).


def longest_two_distinct(a):
    cnt = {}
    left = 0
    best = 0
    for right, v in enumerate(a):
        cnt[v] = cnt.get(v, 0) + 1
        while len(cnt) > 2:
            cnt[a[left]] -= 1
            if cnt[a[left]] == 0:
                del cnt[a[left]]
            left += 1
        best = max(best, right - left + 1)
    return best


if __name__ == "__main__":
    a = [2, 1, 2, 3, 3, 1, 3, 5]
    print(longest_two_distinct(a))
