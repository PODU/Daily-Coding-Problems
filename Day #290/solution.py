# Day 290: Quxes: adjacent different colors merge to third. Smallest remaining count.
# Count r,g,b; distinct<=1 -> total; all same parity -> 2; else 1. Time O(n), Space O(1).
def smallest_quxes(q):
    r = q.count('R')
    g = q.count('G')
    b = q.count('B')
    distinct = (r > 0) + (g > 0) + (b > 0)
    if distinct <= 1:
        return r + g + b
    if r % 2 == g % 2 == b % 2:
        return 2
    return 1


if __name__ == "__main__":
    print(smallest_quxes(['R', 'G', 'B', 'G', 'B']))
