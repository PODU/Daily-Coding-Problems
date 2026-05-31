# Day 1589: Quxes minimization: count R/G/B; two zero counts -> n; all parities equal -> 2; else 1.
# Time O(n), Space O(1).
def min_quxes(a):
    r = a.count('R')
    g = a.count('G')
    b = a.count('B')
    n = len(a)
    zeros = (r == 0) + (g == 0) + (b == 0)
    if zeros >= 2:
        return n
    if (r % 2) == (g % 2) == (b % 2):
        return 2
    return 1


if __name__ == "__main__":
    demo = ['R', 'G', 'B', 'G', 'B']
    print(min_quxes(demo))
