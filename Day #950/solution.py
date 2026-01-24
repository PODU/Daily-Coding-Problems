# Day 950: busiest period - interval with the most people inside the building.
# Sort events by timestamp, sweep maintaining running count. Time O(n log n), Space O(n).

def busiest(events):
    ev = sorted(events, key=lambda e: e["timestamp"])
    cur = 0
    best = None
    ans = (0, 0)
    for i, e in enumerate(ev):
        cur += e["count"] if e["type"] == "enter" else -e["count"]
        next_ts = ev[i + 1]["timestamp"] if i + 1 < len(ev) else e["timestamp"]
        if best is None or cur > best:
            best = cur
            ans = (e["timestamp"], next_ts)
    return ans


if __name__ == "__main__":
    events = [
        {"timestamp": 1526579928, "count": 3, "type": "enter"},
        {"timestamp": 1526579943, "count": 4, "type": "enter"},
        {"timestamp": 1526580382, "count": 2, "type": "exit"},
        {"timestamp": 1526581000, "count": 5, "type": "exit"},
    ]
    print(busiest(events))  # (1526579943, 1526580382)
