# Day 1431: Majority element (appears > floor(n/2)).
# Approach: Boyer-Moore voting. Time: O(n), Space: O(1).


def majority_element(nums):
    count = 0
    candidate = None
    for x in nums:
        if count == 0:
            candidate = x
        count += 1 if x == candidate else -1
    return candidate


if __name__ == "__main__":
    print(majority_element([1, 2, 1, 1, 3, 4, 0]))  # 1
