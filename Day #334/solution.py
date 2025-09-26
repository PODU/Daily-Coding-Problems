# Day 334: 24 Game (fixed order): recursive split of contiguous list, combine with +,-,*,/ (floats).
# Time: O(1) for fixed 4 numbers. Space: O(1).

def solve(a):
    if len(a) == 1:
        return [a[0]]
    res = []
    for i in range(1, len(a)):
        for l in solve(a[:i]):
            for r in solve(a[i:]):
                res.append(l + r)
                res.append(l - r)
                res.append(l * r)
                if abs(r) > 1e-9:
                    res.append(l / r)
    return res


def can24(a):
    return any(abs(v - 24.0) < 1e-6 for v in solve([float(x) for x in a]))


if __name__ == "__main__":
    nums = [5, 2, 7, 8]
    print("True" if can24(nums) else "False")
