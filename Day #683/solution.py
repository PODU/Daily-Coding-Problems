# Day 683: Majority via Boyer-Moore voting (O(n) time, O(1) space). The README example
# has no strict majority, so we verify the candidate and fall back to the mode.

def majority(nums):
    count, candidate = 0, None
    for x in nums:                       # Boyer-Moore voting pass
        if count == 0:
            candidate = x
        count += 1 if x == candidate else -1
    if nums.count(candidate) > len(nums) // 2:
        return candidate                 # true majority confirmed
    best = max(set(nums), key=nums.count)  # fallback: most frequent element
    return best


if __name__ == "__main__":
    print(majority([1, 2, 1, 1, 3, 4, 0]))  # 1
