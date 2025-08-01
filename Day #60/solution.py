# Day 60: Equal-sum partition = subset-sum to total/2, via bitmask DP.
# Time: O(n * sum / word), Space: O(sum) bits.


def can_partition(nums):
    total = sum(nums)
    if total % 2:
        return False
    target = total // 2
    bits = 1  # bit i set => sum i achievable
    for x in nums:
        bits |= bits << x
    return (bits >> target) & 1 == 1


if __name__ == "__main__":
    print(can_partition([15, 5, 20, 10, 35, 15, 10]))  # True
    print(can_partition([15, 5, 20, 10, 35]))           # False
