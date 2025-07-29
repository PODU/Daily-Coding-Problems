# Day 49: Maximum contiguous subarray sum (Kadane), empty subarray allowed.
# Time: O(n), Space: O(1).


def max_subarray(a):
    best = cur = 0  # empty subarray => 0
    for x in a:
        cur = max(0, cur + x)
        best = max(best, cur)
    return best


if __name__ == "__main__":
    print(max_subarray([34, -50, 42, 14, -5, 86]))
    print(max_subarray([-5, -1, -8, -9]))
