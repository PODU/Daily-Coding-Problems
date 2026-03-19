# Day 1231: Estimate pi via Monte Carlo using a shared 64-bit LCG (so every language
# prints the same value). Time O(n) samples, Space O(1).

MASK = (1 << 64) - 1
A = 6364136223846793005
C = 1442695040888963407


def estimate_pi(samples: int, seed: int = 42) -> float:
    x = seed & MASK
    inside = 0
    for _ in range(samples):
        x = (A * x + C) & MASK
        px = (x >> 11) / (1 << 53)
        x = (A * x + C) & MASK
        py = (x >> 11) / (1 << 53)
        if px * px + py * py <= 1.0:
            inside += 1
    return 4.0 * inside / samples


if __name__ == "__main__":
    print(f"{estimate_pi(2_000_000):.3f}")
