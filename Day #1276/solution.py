# Day 1276: Partition a multiset into two subsets of equal sum.
# Subset-sum DP via a bitset (each set bit = reachable sum). Time O(n*S), Space O(S).
def can_partition(nums):
    total = sum(nums)
    if total % 2:
        return False
    target = total // 2
    bits = 1
    for x in nums:
        bits |= bits << x
    return (bits >> target) & 1 == 1


if __name__ == "__main__":
    print(str(can_partition([15, 5, 20, 10, 35, 15, 10])).lower())
    print(str(can_partition([15, 5, 20, 10, 35])).lower())
