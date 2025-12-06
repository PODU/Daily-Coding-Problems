# Day 706: 24 Game (fixed order). Try every parenthesization over the fixed
# sequence, combining sub-results with +,-,*,/. Time ~O(1) for 4 numbers.

def solve(nums):
    if len(nums) == 1:
        return [nums[0]]
    res = []
    for i in range(1, len(nums)):
        for a in solve(nums[:i]):
            for b in solve(nums[i:]):
                res.append(a + b)
                res.append(a - b)
                res.append(a * b)
                if abs(b) > 1e-9:
                    res.append(a / b)
    return res


def game24(digits):
    return any(abs(v - 24.0) < 1e-6 for v in solve([float(x) for x in digits]))


if __name__ == "__main__":
    print("True" if game24([5, 2, 7, 8]) else "False")
