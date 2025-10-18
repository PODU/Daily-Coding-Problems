# Day 456: Busiest period in a building from enter/exit events.
# Sort by timestamp, sweep occupancy, track interval of max. Time O(n log n).


def busiest(events):
    ev = sorted(events, key=lambda e: e["timestamp"])
    cur = 0
    best = -1
    best_start = best_end = None
    n = len(ev)
    for i, e in enumerate(ev):
        cur += e["count"] if e["type"] == "enter" else -e["count"]
        end = ev[i + 1]["timestamp"] if i + 1 < n else e["timestamp"]
        if cur > best:
            best = cur
            best_start, best_end = e["timestamp"], end
    return best_start, best_end


if __name__ == "__main__":
    events = [
        {"timestamp": 1526579928, "count": 3, "type": "enter"},
        {"timestamp": 1526579940, "count": 2, "type": "enter"},
        {"timestamp": 1526580000, "count": 1, "type": "exit"},
        {"timestamp": 1526580382, "count": 4, "type": "exit"},
    ]
    print(busiest(events))  # (1526579940, 1526580000)
