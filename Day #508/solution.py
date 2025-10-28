# Day 508: Approximate median: sample a fixed, sublinear subset (size independent of N),
# return the sample's median -> lands in the central half [N/4, 3N/4] w.h.p.
# Sampling+median: O(s log s), sublinear in N. (Rank shown only for the demo.)

MASK = (1 << 64) - 1
state = 0x0123456789ABCDEF  # fixed seed -> deterministic


def next_rand():
    global state
    state = (state * 6364136223846793005 + 1442695040888963407) & MASK
    return state >> 33  # top 31 bits


def main():
    N = 1000
    SAMPLE_SIZE = 99  # fixed, independent of N (sublinear)

    data = list(range(1, N + 1))
    for i in range(N - 1, 0, -1):
        j = next_rand() % (i + 1)
        data[i], data[j] = data[j], data[i]

    sample = [data[next_rand() % N] for _ in range(SAMPLE_SIZE)]
    sample.sort()
    median = sample[SAMPLE_SIZE // 2]

    rank = sum(1 for v in data if v <= median)

    print(f"Approximate median: {median}")
    print(f"Rank: {rank} (acceptable range: {N // 4} to {3 * N // 4})")


if __name__ == "__main__":
    main()
