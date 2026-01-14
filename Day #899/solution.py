# Day 899: 24 game, fixed order: recursively split the sequence at each position, combine left/right results with +,-,*,/ (eps for div). O(4^n) over splits; O(n) depth.
EPS = 1e-6


def solve(nums, lo, hi):
    if hi - lo == 1:
        return [nums[lo]]
    res = []
    for mid in range(lo + 1, hi):
        for a in solve(nums, lo, mid):
            for b in solve(nums, mid, hi):
                res.append(a + b)
                res.append(a - b)
                res.append(a * b)
                if abs(b) > EPS:
                    res.append(a / b)
    return res


def can_reach_24(nums):
    vals = [float(x) for x in nums]
    return any(abs(v - 24.0) < EPS for v in solve(vals, 0, len(vals)))


if __name__ == "__main__":
    input_nums = [5, 2, 7, 8]
    print("True" if can_reach_24(input_nums) else "False")
