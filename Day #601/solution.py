# Day 601: Reconstruct a permutation of [0..N] matching +/- relations between neighbors.
# Approach: greedy low/high pointers (DI-match). Time O(n), Space O(n). Any consistent array is valid.


def reconstruct(signs):
    # signs[0] is None; signs[k] is '+' if a[k] > a[k-1], else '-'.
    n = len(signs)               # numbers 0..n-1
    low, high = 0, n - 1
    res = []
    for k in range(1, n):
        if signs[k] == '+':
            res.append(low)
            low += 1
        else:
            res.append(high)
            high -= 1
    res.append(low)
    return res


def main():
    signs = [None, '+', '+', '-', '+']
    print(reconstruct(signs))


if __name__ == '__main__':
    main()
