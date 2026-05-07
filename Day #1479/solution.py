# Day 1479: Partition a list into <x, ==x, >x around pivot x.
# Stable bucketing into three lists preserves relative order. Time O(N), Space O(N).
# (An in-place Dutch-national-flag variant is O(1) space but reorders parts.)

def partition(lst, x):
    less, equal, greater = [], [], []
    for v in lst:
        if v < x:
            less.append(v)
        elif v == x:
            equal.append(v)
        else:
            greater.append(v)
    return less + equal + greater


if __name__ == "__main__":
    print(partition([9, 12, 3, 5, 14, 10, 10], 10))
    # [9, 3, 5, 10, 10, 12, 14]
