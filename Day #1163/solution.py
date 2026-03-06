# Day 1163: Greedy: sort intervals by end ascending; keep if start >= last kept end; count removals. O(n log n) time, O(1) extra space.

def erase_overlap_intervals(intervals):
    intervals.sort(key=lambda iv: iv[1])
    removals = 0
    last_end = float('-inf')
    for start, end in intervals:
        if start >= last_end:
            last_end = end
        else:
            removals += 1
    return removals

if __name__ == "__main__":
    intervals = [(7, 9), (2, 4), (5, 8)]
    print(erase_overlap_intervals(intervals))
