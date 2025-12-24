# Day 795: Von Neumann debiasing: toss biased coin twice; (0,1)->0, (1,0)->1, else retry.
# Output is provably fair regardless of bias. O(1) expected tosses per fair bit.
import random


def toss_biased():
    return 1 if random.random() < 0.3 else 0  # P(1) = 0.3


def fair_coin():
    while True:
        a = toss_biased()
        b = toss_biased()
        if a == 0 and b == 1:
            return 0
        if a == 1 and b == 0:
            return 1


def main():
    random.seed(42)
    n = 100000
    ones = sum(fair_coin() for _ in range(n))
    print(f"{ones / n:.1f}")


if __name__ == "__main__":
    main()
