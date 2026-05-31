# Day 1585: Sieve of Eratosthenes + incremental infinite prime generator.
# Sieve marks composites up to N; generator yields primes indefinitely via a sieve map.
# Time: O(N log log N) sieve; Space: O(N).
from itertools import islice


def sieve(n):
    comp = [False] * n
    primes = []
    for i in range(2, n):
        if not comp[i]:
            primes.append(i)
            for j in range(i * i, n, i):
                comp[j] = True
    return primes


def prime_generator():
    """Yield primes indefinitely (incremental sieve, no upper bound)."""
    composites = {}
    n = 2
    while True:
        if n not in composites:
            yield n
            composites[n * n] = [n]
        else:
            for p in composites.pop(n):
                composites.setdefault(n + p, []).append(p)
        n += 1


if __name__ == "__main__":
    print("Primes < 30:", sieve(30))
    print("First 10 primes:", list(islice(prime_generator(), 10)))
