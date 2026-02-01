# Day 1002: Smallest positive integer not expressible as a subset sum (sorted array).
# Track the smallest unreachable value `res` (init 1). If the next element x <= res
# we can extend the reachable range to res+x; otherwise res is the answer. O(N).

def smallest_non_subset_sum(nums):
    res = 1
    for x in nums:
        if x > res:
            break
        res += x
    return res


if __name__ == "__main__":
    print(smallest_non_subset_sum([1, 2, 3, 10]))  # 7
