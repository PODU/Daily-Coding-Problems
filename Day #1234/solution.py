# Day 1234: Min Quxes remaining. Each merge flips parity of all 3 color counts.
# If one color only -> N (no merge). If all counts same parity -> 2, else 1.
# Time O(n), Space O(1).
from collections import Counter


def min_quxes(quxes):
    if not quxes:
        return 0
    c = Counter(quxes)
    if len([x for x in 'RGB' if c[x] > 0]) == 1:
        return len(quxes)
    if c['R'] % 2 == c['G'] % 2 == c['B'] % 2:
        return 2
    return 1


if __name__ == "__main__":
    print(min_quxes(['R', 'G', 'B', 'G', 'B']))
