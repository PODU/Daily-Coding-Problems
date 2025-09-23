# Day 323: Approximate median: median of k random samples (seeded RNG) -> rank in [N/4, 3N/4] whp.
# Time: O(k log k), o(N) for k<N; Space: O(k).
import random


def approx_median(a, k, seed):
    rng = random.Random(seed)
    sample = [a[rng.randrange(len(a))] for _ in range(k)]
    sample.sort()
    return sample[k // 2]


if __name__ == "__main__":
    a = list(range(101))  # N = 101, values 0..100
    N = len(a)
    val = approx_median(a, 15, 42)
    rank = val  # rank in sorted 0..100 equals value
    ok = (N // 4 <= rank) and (rank <= (3 * N) // 4)
    print(f"Approximate median is within [N/4, 3N/4]: {'true' if ok else 'false'}")
