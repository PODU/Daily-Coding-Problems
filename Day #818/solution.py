# Day 818: Sieve of Eratosthenes for primes below N; unbounded incremental sieve (dict of next composites).
# Sieve: O(N log log N). Generator yields primes indefinitely.
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


def gen_primes():
    # Incremental sieve: map next-composite -> prime that generated it.
    composites = {}
    candidate = 2
    while True:
        if candidate not in composites:
            yield candidate
            composites[candidate * candidate] = candidate
        else:
            p = composites.pop(candidate)
            nxt = candidate + p
            while nxt in composites:
                nxt += p
            composites[nxt] = p
        candidate += 1


if __name__ == "__main__":
    print("Primes below 30:", sieve(30))
    print("First 10 primes:", list(islice(gen_primes(), 10)))
