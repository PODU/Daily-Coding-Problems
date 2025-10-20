# Day 463: Minimum Quxes remaining after merges.
# Approach: if only one color present, none can merge -> total count. Else if all three
# color counts share the same parity -> 2, otherwise -> 1. Time: O(n), Space: O(1).
from collections import Counter


def min_quxes(quxes):
    c = Counter(quxes)
    r, g, b = c["R"], c["G"], c["B"]
    present = (r > 0) + (g > 0) + (b > 0)
    if present <= 1:
        return r + g + b  # all same color (or empty)
    if r % 2 == g % 2 == b % 2:
        return 2
    return 1


if __name__ == "__main__":
    quxes = ["R", "G", "B", "G", "B"]
    print(min_quxes(quxes))
