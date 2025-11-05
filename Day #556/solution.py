# Day 556: Non-decreasing with at most one modification: single pass, greedy fix. O(n) time, O(1) space.
def check_possibility(nums):
    nums = list(nums)
    cnt = 0
    for i in range(1, len(nums)):
        if nums[i] < nums[i - 1]:
            cnt += 1
            if cnt > 1:
                return False
            if i < 2 or nums[i - 2] <= nums[i]:
                nums[i - 1] = nums[i]
            else:
                nums[i] = nums[i - 1]
    return True


if __name__ == "__main__":
    print(str(check_possibility([10, 5, 7])).lower())
    print(str(check_possibility([10, 5, 1])).lower())
