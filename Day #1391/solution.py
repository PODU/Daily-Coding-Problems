# Day 1391: Von Neumann fair coin from a biased toss: toss twice, (0,1)->0, (1,0)->1, else retry. O(1) expected per fair toss.
import random

rng = random.Random(12345)


def toss_biased():
    return 1 if rng.random() < 0.3 else 0


def fair_toss():
    while True:
        a, b = toss_biased(), toss_biased()
        if a == 0 and b == 1:
            return 0
        if a == 1 and b == 0:
            return 1


def main():
    n, heads = 100000, 0
    for _ in range(n):
        heads += fair_toss()
    print(f"heads fraction: {heads / n:.2f}")


if __name__ == "__main__":
    main()
