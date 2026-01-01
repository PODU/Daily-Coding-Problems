# Day 836: Fisher-Yates shuffle using only a rand(k) RNG (uniform 1..k) and swaps.
# For i=n-1..1: pick j uniform in 0..i via rand(i+1)-1, swap a[i],a[j]. O(N) time, O(1) extra.
# Unbiased: each step chooses uniformly among i+1 positions, so all n! permutations are equally likely.

class RNG:
    """Deterministic LCG exposing rand(k) -> uniform int in [1, k]."""
    def __init__(self, seed):
        self.state = seed & 0xFFFFFFFFFFFFFFFF

    def _next(self):
        # Numerical Recipes LCG constants.
        self.state = (self.state * 6364136223846793005 + 1442695040888963407) & 0xFFFFFFFFFFFFFFFF
        return self.state >> 16

    def rand(self, k):
        # Uniform in [1, k] via rejection to avoid modulo bias.
        limit = (1 << 48) - ((1 << 48) % k)
        while True:
            r = self._next() & ((1 << 48) - 1)
            if r < limit:
                return r % k + 1


def shuffle(a, rng):
    for i in range(len(a) - 1, 0, -1):
        j = rng.rand(i + 1) - 1  # uniform 0..i
        a[i], a[j] = a[j], a[i]
    return a


if __name__ == "__main__":
    deck = list(range(1, 53))
    rng = RNG(12345)
    shuffle(deck, rng)
    print(" ".join(map(str, deck)))
    assert sorted(deck) == list(range(1, 53))
