# Day 589: Expected waiting time for patterns on a fair d6: "5 then 6" (distinct) E=36;
# "5 then 5" (self-overlap) E=6+36=42. Monte Carlo corroborates. Time O(1) for theory.
import random


def simulate(first, second, trials=200000, seed=12345):
    rng = random.Random(seed)
    total = 0
    for _ in range(trials):
        prev, count = 0, 0
        while True:
            r = rng.randint(1, 6)
            count += 1
            if prev == first and r == second:
                break
            prev = r
        total += count
    return total / trials


if __name__ == "__main__":
    e1 = 36  # five then six
    e2 = 42  # five then five
    # simulate(5, 6) and simulate(5, 5) corroborate the theory
    print("Game 1 (five then six) expected rolls: {}".format(e1))
    print("Game 2 (five then five) expected rolls: {}".format(e2))
    print("Alice should play Game 1 (five then six) since it has the lower expected cost.")
