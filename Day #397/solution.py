# Day 397: Greedy activity selection: sort by end time, pick job if start >= last end (intervals [start,end)).
# Time O(n log n), Space O(n).
from typing import List, Tuple


def select_jobs(jobs: List[Tuple[int, int]]) -> List[Tuple[int, int]]:
    chosen = []
    last_end = float("-inf")
    for start, end in sorted(jobs, key=lambda j: j[1]):
        if start >= last_end:
            chosen.append((start, end))
            last_end = end
    return chosen


if __name__ == "__main__":
    jobs = [(0, 6), (1, 4), (3, 5), (3, 8), (4, 7), (5, 9), (6, 10), (8, 11)]
    chosen = select_jobs(jobs)
    print("[" + ", ".join("({}, {})".format(s, e) for s, e in chosen) + "]")
