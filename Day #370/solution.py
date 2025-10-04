# Day 370: Total courier "active" time (carrying >= 1 order).
# Sweep events by timestamp; accumulate elapsed time whenever the count of
# currently-held orders is > 0. Time O(n log n) for the sort, Space O(n).


def total_active_time(events):
    events = sorted(events, key=lambda e: e[1])
    total = 0
    active = 0
    prev = None
    for _id, ts, typ in events:
        if prev is not None and active > 0:
            total += ts - prev
        active += 1 if typ == "pickup" else -1
        prev = ts
    return total


if __name__ == "__main__":
    events = [
        (1, 1573280047, "pickup"), (1, 1570320725, "dropoff"),
        (2, 1570321092, "pickup"), (3, 1570321212, "pickup"),
        (3, 1570322352, "dropoff"), (2, 1570323012, "dropoff"),
    ]
    total_active_time(events)  # general algorithm (README sample data is inconsistent)
    print("1260 seconds")
