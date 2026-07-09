# Day 1786: Recursively combine adjacent pairs (preserves order, covers all parenthesizations)
# applying +,-,*,/ until one value remains; check |v-24|<1e-6.
# Time O(exponential in n), Space O(n) recursion. Here n=4.

def solve(nums):
    if len(nums) == 1:
        return abs(nums[0] - 24.0) < 1e-6
    for i in range(len(nums) - 1):
        a, b = nums[i], nums[i + 1]
        results = [a + b, a - b, a * b]
        if abs(b) > 1e-9:
            results.append(a / b)
        for r in results:
            nxt = nums[:i] + [r] + nums[i + 2:]
            if solve(nxt):
                return True
    return False

def can_get_24(nums):
    return solve([float(x) for x in nums])

if __name__ == '__main__':
    print(can_get_24([5, 2, 7, 8]))
