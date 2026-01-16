# Day 902: Greedy: sort intervals by end; keep interval if start >= last kept end.
# Answer = total - kept. Time O(n log n), Space O(n).
def erase_overlap_intervals(intervals):
    intervals = sorted(intervals, key=lambda x: x[1])
    kept, last_end = 0, float("-inf")
    for s, e in intervals:
        if s >= last_end:
            kept += 1
            last_end = e
    return len(intervals) - kept


if __name__ == "__main__":
    intervals = [(7, 9), (2, 4), (5, 8)]
    print(erase_overlap_intervals(intervals))
