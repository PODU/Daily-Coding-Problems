# Day 1030: Quxes minimum remaining. Count colors; parity-based O(N) formula.
# If two counts are 0 -> n; else if all parities equal -> 2; else -> 1. Time O(N), Space O(1).


def min_quxes(q):
    r = q.count('R')
    g = q.count('G')
    b = q.count('B')
    n = r + g + b
    zeros = (r == 0) + (g == 0) + (b == 0)
    if zeros >= 2:
        return n
    if r % 2 == g % 2 == b % 2:
        return 2
    return 1


if __name__ == "__main__":
    print(min_quxes(['R', 'G', 'B', 'G', 'B']))
