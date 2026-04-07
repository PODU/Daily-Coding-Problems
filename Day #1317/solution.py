# Day 1317: Reconstruct a permutation of [0..N] consistent with up/down signs.
# Two-pointer (DI-match): '+' takes next low, '-' takes next high. Time O(N).
# Any consistent array is valid; README shows one such answer.

def reconstruct(signs):
    # signs[0] is None; signs[i] in {'+','-'} for i>=1
    n = len(signs)
    N = n - 1
    res = []
    lo, hi = 0, N
    for i in range(1, n):
        if signs[i] == '+':
            res.append(lo)
            lo += 1
        else:
            res.append(hi)
            hi -= 1
    res.append(lo)  # lo == hi for the final element
    return res


def consistent(signs, a):
    for i in range(1, len(signs)):
        if signs[i] == '+' and not a[i] > a[i - 1]:
            return False
        if signs[i] == '-' and not a[i] < a[i - 1]:
            return False
    return True


if __name__ == "__main__":
    signs = [None, '+', '+', '-', '+']
    a = reconstruct(signs)
    print(a, " consistent=" + str(consistent(signs, a)))
    # -> [0, 1, 4, 2, 3]  (README's [1, 2, 3, 0, 4] is another valid answer)
