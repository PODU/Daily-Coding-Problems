# Day 933: Reconstruct a permutation of [0..N] consistent with +/- signs.
# Two-pointer: '+' takes the current low, '-' takes the current high. O(n) time, O(n) space.
# Many arrangements are valid; README shows [1,2,3,0,4], this prints another valid one.


def reconstruct(signs):
    # signs[0] is None; signs[i] in {'+','-'} for i>=1.
    n = len(signs)
    lo, hi = 0, n - 1
    res = []
    for i in range(1, n):
        if signs[i] == '+':
            res.append(lo)
            lo += 1
        else:
            res.append(hi)
            hi -= 1
    res.append(lo)  # lo == hi
    return res


if __name__ == "__main__":
    signs = [None, '+', '+', '-', '+']
    print(reconstruct(signs))  # e.g. [0, 1, 4, 2, 3] (a valid reconstruction)
