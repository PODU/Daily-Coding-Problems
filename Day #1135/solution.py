# Day 1135: Power set via bitmask enumeration, sorted by (size, lexicographic). O(2^n * n).
def power_set(nums):
    n = len(nums)
    subsets = []
    for mask in range(1 << n):
        subsets.append([nums[i] for i in range(n) if mask & (1 << i)])
    subsets.sort(key=lambda s: (len(s), s))
    return subsets


def fmt(subsets):
    return "[" + ", ".join("[" + ", ".join(map(str, s)) + "]" for s in subsets) + "]"


if __name__ == "__main__":
    print(fmt(power_set([1, 2, 3])))
