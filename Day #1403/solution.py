# Day 1403: Monte-Carlo simulation of two stop conditions plus exact theory.
# E[rolls until 5->6] = 36 (distinct faces); E[rolls until 5->5] = 42 (same face).
# Time O(trials * rolls), Space O(1).
import random


def play(second, rng):
    rolls, prev = 0, 0
    while True:
        cur = rng.randint(1, 6)
        rolls += 1
        if prev == 5 and cur == second:
            return rolls
        prev = cur


if __name__ == "__main__":
    rng = random.Random(42)
    trials = 200000
    avg56 = sum(play(6, rng) for _ in range(trials)) / trials
    avg55 = sum(play(5, rng) for _ in range(trials)) / trials
    print(f"Game 1 (five then six): simulated avg = {avg56:.2f}, theoretical = 36")
    print(f"Game 2 (five then five): simulated avg = {avg55:.2f}, theoretical = 42")
    print("Alice should play Game 1 (five-then-six): fewer expected rolls, less pay.")
