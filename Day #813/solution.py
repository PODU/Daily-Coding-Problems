# Day 813: Non-decreasing with at most 1 modification: single pass counting violations,
# greedily lower nums[i-1] or raise nums[i]. Time O(n), Space O(1).


def can_be_non_decreasing(nums):
    nums = list(nums)
    count = 0
    for i in range(1, len(nums)):
        if nums[i - 1] > nums[i]:
            count += 1
            if count > 1:
                return False
            if i < 2 or nums[i - 2] <= nums[i]:
                nums[i - 1] = nums[i]
            else:
                nums[i] = nums[i - 1]
    return True


def main():
    print(str(can_be_non_decreasing([10, 5, 7])).lower())
    print(str(can_be_non_decreasing([10, 5, 1])).lower())


if __name__ == "__main__":
    main()
