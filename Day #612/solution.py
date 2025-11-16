# Day 612: Min intervals to remove so the rest are non-overlapping (touching allowed).
# Approach: sort by end, greedily keep max non-overlapping; answer = total - kept. Time O(n log n).


def min_removals(intervals):
    intervals = sorted(intervals, key=lambda iv: iv[1])
    kept, end = 0, float('-inf')
    for s, e in intervals:
        if s >= end:
            kept += 1
            end = e
    return len(intervals) - kept


def main():
    intervals = [(7, 9), (2, 4), (5, 8)]
    print(min_removals(intervals))  # 1


if __name__ == '__main__':
    main()
