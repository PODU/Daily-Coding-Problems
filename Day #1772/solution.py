# Day 1772: Min jumps to reach N. Grow k until triangular sum >= |N| and (sum-|N|) even.
# Flipping any jump j changes parity of (sum-N) by 2j, so even surplus is reachable. O(sqrt(N)).


def min_jumps(n):
    s = abs(n)
    total = 0
    k = 0
    while total < s or (total - s) % 2 != 0:
        k += 1
        total += k
    return k


if __name__ == "__main__":
    print(min_jumps(10))
