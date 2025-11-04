# Day 551: Von Neumann fair-coin from biased coin: toss pairs, (0,1)->0, (1,0)->1, else retry.
# Expected O(1) tosses per fair toss; O(1) space.

rng_state = 88172645463325252


def next_uniform():  # xorshift64 -> [0,1)
    global rng_state
    x = rng_state & 0xFFFFFFFFFFFFFFFF
    x ^= (x << 13) & 0xFFFFFFFFFFFFFFFF
    x ^= x >> 7
    x ^= (x << 17) & 0xFFFFFFFFFFFFFFFF
    rng_state = x
    return (x >> 11) * (1.0 / 9007199254740992.0)


def toss_biased():  # p(1) = 0.3
    return 1 if next_uniform() < 0.3 else 0


def fair_toss():
    while True:
        a = toss_biased()
        b = toss_biased()
        if a == 0 and b == 1:
            return 0
        if a == 1 and b == 0:
            return 1


def main():
    heads = tails = 0
    for _ in range(100000):
        if fair_toss() == 1:
            heads += 1
        else:
            tails += 1
    print(f"heads: {heads}, tails: {tails}")


if __name__ == "__main__":
    main()
