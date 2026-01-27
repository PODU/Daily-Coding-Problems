# Day 967: Find the duplicate in array of n+1 ints from {1..n}.
# Approach: Floyd's tortoise & hare on value->index cycle. Time O(n), Space O(1).


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


if __name__ == '__main__':
    print(find_duplicate([1, 3, 4, 2, 2]))  # 2
