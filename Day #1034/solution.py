# Day 1034: Expected rounds until one of n fair coins remains.
# Model: expected max of n iid Geometric(1/2) lifetimes; E = sum_{m>=0}(1-(1-2^-m)^n). O(iter).


def expected_rounds(n):
    e = 0.0
    p = 1.0  # p = 2^-m
    m = 0
    while m < 100000:
        term = 1.0 - (1.0 - p) ** n
        if term < 1e-12 and m > 0:
            break
        e += term
        p *= 0.5
        m += 1
    return e


if __name__ == "__main__":
    n = 4
    print("n=%d -> expected rounds: %.4f" % (n, expected_rounds(n)))
