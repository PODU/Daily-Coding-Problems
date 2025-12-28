# Day 815: Monte Carlo pi: sample (x,y) in unit square via deterministic splitmix64 RNG,
# pi ~= 4*inside/total. Fixed seed -> deterministic. Time O(N), Space O(1).
# Uses a numpy-vectorized splitmix64 when numpy is available (bit-identical
# sequence to the scalar fallback), since 20M big-int hashes are slow in pure Python.
import sys

try:
    sys.stdout.reconfigure(encoding="utf-8")
except Exception:
    pass

MASK = (1 << 64) - 1
GAMMA = 0x9E3779B97F4A7C15
MIX1 = 0xBF58476D1CE4E5B9
MIX2 = 0x94D049BB133111EB


def make_rng(seed):
    state = seed & MASK

    def next_double():
        nonlocal state
        state = (state + GAMMA) & MASK
        z = state
        z = ((z ^ (z >> 30)) * MIX1) & MASK
        z = ((z ^ (z >> 27)) * MIX2) & MASK
        z = z ^ (z >> 31)
        return (z >> 11) * (1.0 / 9007199254740992.0)

    return next_double


def count_inside_python(seed, n):
    rng = make_rng(seed)
    inside = 0
    for _ in range(n):
        x = rng()
        y = rng()
        if x * x + y * y <= 1.0:
            inside += 1
    return inside


def count_inside_numpy(np, seed, n):
    gamma, mix1, mix2 = np.uint64(GAMMA), np.uint64(MIX1), np.uint64(MIX2)
    s30, s27, s31, s11 = (np.uint64(k) for k in (30, 27, 31, 11))
    scale = 1.0 / 9007199254740992.0
    inside = 0
    chunk = 1 << 20
    for lo in range(0, 2 * n, 2 * chunk):
        count = min(2 * chunk, 2 * n - lo)
        i = np.arange(lo + 1, lo + count + 1, dtype=np.uint64)
        z = np.uint64(seed) + i * gamma
        z = (z ^ (z >> s30)) * mix1
        z = (z ^ (z >> s27)) * mix2
        z = z ^ (z >> s31)
        d = (z >> s11).astype(np.float64) * scale
        x, y = d[0::2], d[1::2]
        inside += int(np.count_nonzero(x * x + y * y <= 1.0))
    return inside


def main():
    N = 10_000_000
    try:
        import numpy as np
        inside = count_inside_numpy(np, 42, N)
    except ImportError:
        inside = count_inside_python(42, N)
    pi = 4.0 * inside / N
    print(f"Estimated pi ≈ {pi:.3f}")


if __name__ == "__main__":
    main()
