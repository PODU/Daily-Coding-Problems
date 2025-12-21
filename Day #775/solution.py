# Day 775: Minimum meeting rooms = max overlapping intervals.
# Sort starts and ends, two-pointer sweep. O(n log n) time, O(n) space.


def min_rooms(intervals):
    starts = sorted(s for s, _ in intervals)
    ends = sorted(e for _, e in intervals)
    rooms = best = i = j = 0
    n = len(intervals)
    while i < n:
        if starts[i] < ends[j]:
            rooms += 1
            i += 1
            best = max(best, rooms)
        else:
            rooms -= 1
            j += 1
    return best


if __name__ == "__main__":
    print(min_rooms([(30, 75), (0, 50), (60, 150)]))  # 2
