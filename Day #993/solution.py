# Day 993: Majority element (the value occurring more than floor(n/2) times).
# Count occurrences in a hash map and return the most frequent value. This also
# yields the expected answer for the README's (non-strict) example. O(n) time/space.
from collections import Counter


def majority(nums):
    return Counter(nums).most_common(1)[0][0]


if __name__ == "__main__":
    print(majority([1, 2, 1, 1, 3, 4, 0]))  # 1
