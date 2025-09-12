# Day 260: Reconstruct a permutation of [0..N] consistent with +/- sign array.
# Grow a list: on '+' append max+1, on '-' append min-1; shift by -min into [0..N].
# O(n) time, O(n) space.


def reconstruct(signs):  # signs[0] is None
    res = [0]
    cur_max = cur_min = 0
    for s in signs[1:]:
        if s == '+':
            cur_max += 1
            res.append(cur_max)
        else:
            cur_min -= 1
            res.append(cur_min)
    off = -cur_min
    return [x + off for x in res]


if __name__ == "__main__":
    signs = [None, '+', '+', '-', '+']
    print(reconstruct(signs))
