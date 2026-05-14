# Day 1514: Sliding window with last-seen index map; advance left past duplicates, track max window length.
# Time: O(n), Space: O(n).
def longest_distinct(a):
    last = {}
    best = 0
    left = 0
    for r, v in enumerate(a):
        if v in last and last[v] >= left:
            left = last[v] + 1
        last[v] = r
        best = max(best, r - left + 1)
    return best


if __name__ == "__main__":
    print(longest_distinct([5, 1, 3, 5, 2, 3, 4, 1]))
