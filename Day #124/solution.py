# Day 124: Expected flipping rounds until one coin remains.
# DP: E[n](1-2^-n) = 1 + sum_{k<n} P(k survive)*E[k]. O(n^2) time, O(n) space.


def expected_rounds(n):
    E = [0.0] * (n + 1)
    for m in range(2, n + 1):
        p = 0.5 ** m  # p_0
        s = 0.0
        for k in range(m):
            s += p * E[k]
            p = p * (m - k) / (k + 1)
        pn = 0.5 ** m
        E[m] = (1.0 + s) / (1.0 - pn)
    return E[n] if n >= 1 else 0.0


if __name__ == "__main__":
    for n in [1, 2, 3, 4, 5, 10]:
        print("n=%-2d expected rounds = %.6f" % (n, expected_rounds(n)))
