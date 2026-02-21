# Day 1109: Three-way (Dutch national flag) partition around pivot x.
# Bucket into <x, ==x, >x preserving relative order to match the example.
# Time: O(N). Space: O(N).
def partition_three(lst, x):
    less = [v for v in lst if v < x]
    equal = [v for v in lst if v == x]
    greater = [v for v in lst if v > x]
    return less + equal + greater


if __name__ == "__main__":
    print(partition_three([9, 12, 3, 5, 14, 10, 10], 10))  # [9, 3, 5, 10, 10, 12, 14]
