# Day 1579: Maximum circular subarray sum (empty allowed -> 0).
# ans = max(Kadane-with-empty, total - minSubarray). Time: O(n); Space: O(1).


def max_circular(a):
    total = max_end = max_sum = 0
    min_end = 0
    min_sum = float("inf")
    for x in a:
        total += x
        max_end = max(x, max_end + x)
        max_sum = max(max_sum, max_end)
        min_end = min(x, min_end + x)
        min_sum = min(min_sum, min_end)
    return max(max_sum, total - min_sum)


if __name__ == "__main__":
    print(max_circular([8, -1, 3, 4]))  # 15
    print(max_circular([-4, 5, 1, 0]))  # 6
