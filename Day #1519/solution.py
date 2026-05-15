# Day 1519: Two dice games via Monte Carlo simulation. Expected rolls: "5 then 6"=36, "5 then 5"=42.
# Time: O(trials * rolls_per_trial). Space: O(1).
import random


def simulate(first, second, trials, rng):
    rand = rng.random
    total = 0
    for _ in range(trials):
        prev, rolls = 0, 0
        while True:
            r = int(rand() * 6.0) + 1
            rolls += 1
            if prev == first and r == second:
                break
            prev = r
        total += rolls
    return total / trials


def main():
    rng = random.Random(12345)
    trials = 500000
    e1 = simulate(5, 6, trials, rng)
    e2 = simulate(5, 5, trials, rng)
    print(f"Game 1 (five then six) expected rolls: {e1:.2f}")
    print(f"Game 2 (five then five) expected rolls: {e2:.2f}")
    print("Alice should play: Game 1 (five then six)")


if __name__ == "__main__":
    main()
