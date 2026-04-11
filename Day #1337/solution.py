# Day 1337: Sieve of Eratosthenes: mark multiples of each prime from p*p. O(N log log N) time, O(N) space.
# Bonus: an incremental-sieve generator that yields primes indefinitely.


def sieve(n):
    is_prime = [True] * max(n, 2)
    is_prime[0] = is_prime[1] = False
    for p in range(2, int(n ** 0.5) + 1):
        if is_prime[p]:
            for m in range(p * p, n, p):
                is_prime[m] = False
    return [i for i in range(2, n) if is_prime[i]]


def prime_generator():
    """Yield primes indefinitely using an incremental sieve."""
    composites = {}
    candidate = 1
    while True:
        candidate += 1
        if candidate not in composites:
            composites[candidate * candidate] = [candidate]
            yield candidate
        else:
            for p in composites.pop(candidate):
                composites.setdefault(candidate + p, []).append(p)


def main():
    print(" ".join(map(str, sieve(100))))

    gen = prime_generator()
    print(" ".join(str(next(gen)) for _ in range(10)))


if __name__ == "__main__":
    main()
