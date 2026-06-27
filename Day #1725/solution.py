# Day 1725: Min intervals to remove for non-overlapping (touching allowed).
# Greedy: sort by end, keep intervals whose start >= last kept end. O(n log n) time, O(1) extra space.

def min_removals(intervals):
    intervals = sorted(intervals, key=lambda iv: iv[1])
    kept, last_end = 0, float("-inf")
    for start, end in intervals:
        if start >= last_end:
            kept += 1
            last_end = end
    return len(intervals) - kept


if __name__ == "__main__":
    intervals = [(7, 9), (2, 4), (5, 8)]
    print(min_removals(intervals))
