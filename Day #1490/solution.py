# Day 1490: Find the duplicate in array of length n+1 with values in {1..n}.
# Floyd's tortoise-and-hare cycle detection. Time O(n), Space O(1).


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
    nums = [1, 2, 3, 4, 2]  # n = 4
    print(find_duplicate(nums))  # expected: 2
