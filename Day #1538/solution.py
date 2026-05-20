# Day 1538: Minimum rooms for overlapping intervals: sort starts & ends, sweep with two pointers.
# Time O(n log n), Space O(n).
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
    print(min_rooms([(30, 75), (0, 50), (60, 150)]))
