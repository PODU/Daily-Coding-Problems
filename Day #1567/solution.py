# Day 1567: Busiest period: sort events by timestamp, sweep current occupancy, track max interval.
# Time O(n log n), space O(n).
def busiest_period(events):
    # events: list of (timestamp, count, is_enter)
    events = sorted(events, key=lambda e: e[0])
    cur = 0
    best = -1
    best_start = best_end = 0
    for i, (ts, count, is_enter) in enumerate(events):
        cur += count if is_enter else -count
        if cur > best and i + 1 < len(events):
            best = cur
            best_start = ts
            best_end = events[i + 1][0]
    return (best_start, best_end)


if __name__ == "__main__":
    events = [
        (1, 3, True),
        (4, 2, True),
        (6, 5, False),
    ]
    start, end = busiest_period(events)
    print("({}, {})".format(start, end))
