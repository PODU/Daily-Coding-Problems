# Day 1260: Busiest period: sort events by timestamp, sweep occupancy, track max-occupancy interval [t_i, t_{i+1}). O(n log n) time, O(n) space.

def busiest_period(events):
    events = sorted(events, key=lambda e: e["timestamp"])
    current = 0
    max_occ = -1
    best = (0, 0)
    for i, e in enumerate(events):
        if e["type"] == "enter":
            current += e["count"]
        else:
            current -= e["count"]
        next_ts = events[i + 1]["timestamp"] if i + 1 < len(events) else e["timestamp"]
        if current > max_occ:
            max_occ = current
            best = (e["timestamp"], next_ts)
    return best


if __name__ == "__main__":
    events = [
        {"timestamp": 1526579928, "count": 3, "type": "enter"},
        {"timestamp": 1526580000, "count": 2, "type": "enter"},
        {"timestamp": 1526580382, "count": 2, "type": "exit"},
        {"timestamp": 1526580500, "count": 1, "type": "enter"},
        {"timestamp": 1526580700, "count": 4, "type": "exit"},
    ]
    start, end = busiest_period(events)
    print("({}, {})".format(start, end))
