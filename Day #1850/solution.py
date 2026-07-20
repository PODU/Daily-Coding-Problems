# Day 1850: rand7() from rand5() via rejection sampling on the 1..25 grid.
# Expected O(1) amortized calls (acceptance 21/25); uniform over 1..7.
import random

random.seed(12345)


def rand5():
    return random.randint(1, 5)


def rand7():
    while True:
        v = 5 * (rand5() - 1) + rand5()  # uniform in 1..25
        if v <= 21:
            return (v - 1) % 7 + 1


def main():
    counts = [0] * 8
    for _ in range(70000):
        counts[rand7()] += 1
    print("Sample of 10:", " ".join(str(rand7()) for _ in range(10)))
    print("Histogram over 70000 draws (each ~10000):")
    for i in range(1, 8):
        print(f"  {i}: {counts[i]}")


if __name__ == "__main__":
    main()
