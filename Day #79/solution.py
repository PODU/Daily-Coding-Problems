# Day 79: Can array become non-decreasing with at most one modification?
# Greedy: on each violation, prefer lowering nums[i]; allow only one fix. Time O(n), Space O(1).


def check_possibility(nums):
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


if __name__ == "__main__":
    print(str(check_possibility([10, 5, 7])).lower())  # true
    print(str(check_possibility([10, 5, 1])).lower())  # false
