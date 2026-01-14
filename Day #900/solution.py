# Day 900: Kadane's max subarray sum; empty subarray (0) allowed so answer is never negative. O(N) time, O(1) space.


def max_subarray_sum(a):
    best = cur = 0
    for x in a:
        cur = max(0, cur + x)
        best = max(best, cur)
    return best


if __name__ == "__main__":
    print(max_subarray_sum([34, -50, 42, 14, -5, 86]))  # 137
    print(max_subarray_sum([-5, -1, -8, -9]))            # 0
