# Day 1436: Length of longest subarray with all distinct elements.
# Approach: Sliding window with last-seen index map; shrink left on repeat.
# Time: O(n), Space: O(n).


def longest_distinct(arr):
    last = {}
    best = left = 0
    for right, v in enumerate(arr):
        if v in last and last[v] >= left:
            left = last[v] + 1
        last[v] = right
        best = max(best, right - left + 1)
    return best


if __name__ == "__main__":
    print(longest_distinct([5, 1, 3, 5, 2, 3, 4, 1]))  # 5
