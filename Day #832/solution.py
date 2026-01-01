# Day 832: Longest subarray with all distinct elements.
# Sliding window with last-seen index map. Time: O(N), Space: O(N).


def longest_distinct(a):
    last = {}
    best = 0
    start = 0
    for i, x in enumerate(a):
        if x in last and last[x] >= start:
            start = last[x] + 1
        last[x] = i
        best = max(best, i - start + 1)
    return best


if __name__ == "__main__":
    print(longest_distinct([5, 1, 3, 5, 2, 3, 4, 1]))
