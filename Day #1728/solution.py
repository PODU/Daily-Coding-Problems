# Day 1728: Simulate a fair coin from a biased one (Von Neumann trick).
# Toss biased coin twice; (0,1)->0, (1,0)->1, else retry. Expected P(heads) ~= 0.5.
# Time: O(1) expected tosses per fair() call. Space: O(1).
import random

rng = random.Random(12345)


def toss_biased():
    # Biased coin: returns 1 with probability p (= 0.3), else 0.
    return 1 if rng.random() < 0.3 else 0


def fair():
    # Von Neumann: extract a fair bit from the biased coin.
    while True:
        a = toss_biased()
        b = toss_biased()
        if a == 0 and b == 1:
            return 0
        if a == 1 and b == 0:
            return 1


def main():
    N = 100000
    heads = sum(fair() for _ in range(N))
    ratio = heads / N
    print(f"Fair coin over {N} tosses, P(heads) ~= {ratio:.2f}")


if __name__ == "__main__":
    main()
