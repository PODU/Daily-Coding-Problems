# Day 21: Meeting rooms: sort starts & ends, sweep with two pointers tracking overlap.
# Time O(n log n), Space O(n).
def min_rooms(intervals):
    n = len(intervals)
    starts = sorted(i[0] for i in intervals)
    ends = sorted(i[1] for i in intervals)
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
    print(min_rooms(intervals))
