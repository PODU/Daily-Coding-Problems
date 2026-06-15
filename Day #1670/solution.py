# Day 1670: Prefix-sum hashmap: store sum->index; if sum-K seen, slice between. O(n) time, O(n) space.
def subarray_sum(a, K):
    seen = {0: -1}; s = 0
    for i, v in enumerate(a):
        s += v
        if s - K in seen:
            return a[seen[s-K]+1:i+1]
        seen.setdefault(s, i)
    return []

print(subarray_sum([1, 2, 3, 4, 5], 9))
