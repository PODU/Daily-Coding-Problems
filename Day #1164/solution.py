# Day 1164: E[T] = sum_t (1 - (1-p)^n - n*p*(1-p)^(n-1)), p=2^-t (P(>1 coin alive after t rounds)). Sum until negligible. O(iterations) time, O(1) space.

def expected_rounds(n):
    total = 0.0
    for t in range(1000):
        p = 2.0 ** (-t)
        q = 1.0 - p
        term = 1.0 - q ** n - n * p * q ** (n - 1)
        total += term
        if t > 0 and term < 1e-15:
            break
    return total

if __name__ == "__main__":
    n = 4
    print("Expected rounds for n={}: {:.4f}".format(n, expected_rounds(n)))
