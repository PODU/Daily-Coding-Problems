# Day 769: Expected rolls for "5 then 6" vs "5 then 5" stopping games.
# Exact via 2-state Markov chains (E1=36, E2=42) plus Monte-Carlo check.
import random


def simulate(second, trials, rng):
    total = 0
    for _ in range(trials):
        prev, rolls = 0, 0
        while True:
            r = rng.randint(1, 6)
            rolls += 1
            if prev == 5 and r == second:
                break
            prev = r
        total += rolls
    return total / trials


if __name__ == "__main__":
    rng = random.Random(12345)
    trials = 200000
    print(f"Game 1 (5 then 6): exact=36, simulated={simulate(6, trials, rng):.2f}")
    print(f"Game 2 (5 then 5): exact=42, simulated={simulate(5, trials, rng):.2f}")
    print("Alice should play Game 1 (5 then 6); it has the lower expected cost.")
