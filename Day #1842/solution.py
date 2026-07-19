# Day 1842: Majority / most-frequent element via a frequency count.
# (Equals the strict majority element whenever one exists.) Time O(N), Space O(N).
from collections import Counter


def majority(a):
    return Counter(a).most_common(1)[0][0]


def main():
    print(majority([1, 2, 1, 1, 3, 4, 0]))  # 1


if __name__ == "__main__":
    main()
