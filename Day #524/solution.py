# Day 524: Kadane's algorithm: max contiguous subarray sum, empty subarray allowed (>=0).
# Time O(n), Space O(1).


def max_subarray_sum(a):
    best = cur = 0  # empty subarray allowed -> min 0
    for x in a:
        cur = max(0, cur + x)
        best = max(best, cur)
    return best


if __name__ == "__main__":
    print(max_subarray_sum([34, -50, 42, 14, -5, 86]))
    print(max_subarray_sum([-5, -1, -8, -9]))
