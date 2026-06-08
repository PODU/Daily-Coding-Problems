# Day 1629: Three-way stable partition around pivot x: collect <x, ==x, >x in order, concat.
# Time O(n), Space O(n).
def partition3(x, lst):
    less = [v for v in lst if v < x]
    equal = [v for v in lst if v == x]
    greater = [v for v in lst if v > x]
    return less + equal + greater


if __name__ == "__main__":
    print(partition3(10, [9, 12, 3, 5, 14, 10, 10]))
