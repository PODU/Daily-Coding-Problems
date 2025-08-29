# Day 192: Jump game -- can we reach the end (each value caps the jump length).
# Greedy farthest-reach. Time O(n), Space O(1).
from typing import List


def can_reach_end(a: List[int]) -> bool:
    reach = 0
    for i, x in enumerate(a):
        if i > reach:
            return False
        reach = max(reach, i + x)
    return True


if __name__ == "__main__":
    print(str(can_reach_end([1, 3, 1, 2, 0, 1])).lower())
    print(str(can_reach_end([1, 2, 1, 0, 0])).lower())
