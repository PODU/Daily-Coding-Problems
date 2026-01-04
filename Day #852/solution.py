# Day 852: maximum circular subarray sum (empty allowed -> 0).
# answer = max(maxKadane clamped at 0, total - minKadane). O(n) time, O(1) space.

def max_circular(a):
    total = 0
    max_k = float("-inf")
    min_k = float("inf")
    cur_max = cur_min = 0
    for x in a:
        total += x
        cur_max = max(x, cur_max + x)
        max_k = max(max_k, cur_max)
        cur_min = min(x, cur_min + x)
        min_k = min(min_k, cur_min)
    non_wrap = max(0, max_k)         # empty subarray allowed
    wrap = total - min_k             # remove the minimum subarray
    return max(non_wrap, wrap)


if __name__ == "__main__":
    print(max_circular([8, -1, 3, 4]))  # 15
    print(max_circular([-4, 5, 1, 0]))  # 6
