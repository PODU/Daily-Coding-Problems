# Day 164: Find duplicate via Floyd's cycle detection (values as next-pointers).
# O(n) time, O(1) extra space (beats the O(n)-space requirement).


def find_duplicate(nums):
    slow = fast = nums[0]
    while True:
        slow = nums[slow]
        fast = nums[nums[fast]]
        if slow == fast:
            break
    slow = nums[0]
    while slow != fast:
        slow = nums[slow]
        fast = nums[fast]
    return slow


if __name__ == "__main__":
    nums = [1, 2, 3, 4, 2]  # n = 4, length n+1
    print(find_duplicate(nums))  # 2
