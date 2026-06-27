# Day 1722: Approximate median via random sampling.
# Sample a sublinear number of elements (~constant), return their exact median.
# With high probability its rank lies in [N/4, 3N/4]. Time: O(s log s) << O(N), Space: O(s).
import random


def approx_median(a, rng):
    n = len(a)
    s = min(n, 99)  # sublinear sample size
    sample = sorted(a[rng.randrange(n)] for _ in range(s))
    return sample[s // 2]


def main():
    # Demo: values 0..99 shuffled deterministically.
    N = 100
    a = list(range(N))
    rng = random.Random(42)
    rng.shuffle(a)

    m = approx_median(a, rng)
    rank = sum(1 for x in a if x < m)
    print(f"Approximate median: {m} (rank {rank} within [N/4, 3N/4] = "
          f"[{N // 4}, {3 * N // 4}])")


if __name__ == "__main__":
    main()
