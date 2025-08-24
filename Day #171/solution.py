# Day 171: Busiest period: sort events by timestamp, sweep occupancy, track max-occupancy interval. O(n log n) time, O(n) space.

ENTER, EXIT = "enter", "exit"

def busiest_period(events):
    events = sorted(events, key=lambda e: e[0])
    occ = 0
    max_occ = -1
    best = (0, 0)
    for i, (ts, count, etype) in enumerate(events):
        occ += count if etype == ENTER else -count
        if occ > max_occ and i + 1 < len(events):
            max_occ = occ
            best = (ts, events[i + 1][0])
    return best

def main():
    events = [
        (1526579928, 3, ENTER),
        (1526580382, 2, EXIT),
        (1526579999, 1, ENTER),
        (1526580001, 5, ENTER),
    ]
    start, end = busiest_period(events)
    print("({}, {})".format(start, end))

if __name__ == "__main__":
    main()
