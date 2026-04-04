# Day 1300: Find a contiguous subarray summing to K (handles negatives).
# Prefix-sum hashmap: for each prefix p, look for p-K seen earlier. O(N) time, O(N) space.


def subarray_sum(a, K):
    first_index = {0: -1}
    prefix = 0
    for j, v in enumerate(a):
        prefix += v
        if prefix - K in first_index:
            return a[first_index[prefix - K] + 1:j + 1]
        first_index.setdefault(prefix, j)
    return []


if __name__ == "__main__":
    print(subarray_sum([1, 2, 3, 4, 5], 9))  # [2, 3, 4]
