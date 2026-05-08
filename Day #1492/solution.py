# Day 1492: Minimum meeting rooms for overlapping intervals.
# Sort starts and ends, two-pointer sweep. Time O(n log n), Space O(n).


def min_rooms(intervals):
    starts = sorted(s for s, _ in intervals)
    ends = sorted(e for _, e in intervals)
    n = len(intervals)
    rooms = max_rooms = j = 0
    for i in range(n):
        while j < n and ends[j] <= starts[i]:
            rooms -= 1
            j += 1
        rooms += 1
        max_rooms = max(max_rooms, rooms)
    return max_rooms


if __name__ == "__main__":
    intervals = [(30, 75), (0, 50), (60, 150)]
    print(min_rooms(intervals))  # expected: 2
