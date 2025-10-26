# Day 494: Maximum circular subarray sum (empty allowed => 0).
# max(maxKadane, total - minKadane); if all negative answer is 0. Time O(n), Space O(1).
def max_circular_subarray(a):
    total = 0
    max_k = float("-inf")
    cur_max = 0
    min_k = float("inf")
    cur_min = 0
    for x in a:
        total += x
        cur_max = max(x, cur_max + x)
        max_k = max(max_k, cur_max)
        cur_min = min(x, cur_min + x)
        min_k = min(min_k, cur_min)
    if max_k < 0:  # all negative -> empty subarray
        return 0
    return max(max_k, total - min_k)


if __name__ == "__main__":
    print(max_circular_subarray([8, -1, 3, 4]))  # 15
    print(max_circular_subarray([-4, 5, 1, 0]))  # 6
