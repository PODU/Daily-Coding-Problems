# Day 143: Partition around pivot into <x, ==x, >x. Stable bucket collect to match expected order. O(n) time, O(n) space.


def partition(lst, x):
    less = [v for v in lst if v < x]
    equal = [v for v in lst if v == x]
    greater = [v for v in lst if v > x]
    return less + equal + greater


if __name__ == "__main__":
    lst = [9, 12, 3, 5, 14, 10, 10]
    print(partition(lst, 10))  # [9, 3, 5, 10, 10, 12, 14]
