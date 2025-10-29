# Day 513: Contiguous subarray summing to K via prefix-sum hash map. O(n) time, O(n) space.
def subarray_sum(a, K):
    seen = {0: -1}
    pre = 0
    for i, v in enumerate(a):
        pre += v
        if pre - K in seen:
            return a[seen[pre - K] + 1:i + 1]
        seen.setdefault(pre, i)
    return []


if __name__ == "__main__":
    print(subarray_sum([1, 2, 3, 4, 5], 9))
