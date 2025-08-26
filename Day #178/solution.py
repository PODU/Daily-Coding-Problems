# Day 178: Monte Carlo simulation of two dice stopping games; average rolls. O(trials) time, O(1) space.
# Theory: E[rolls "5 then 6"]=36, E[rolls "5 then 5"]=42. Exact sim values depend on RNG/seed.
import random

def trial(second):
    rolls, prev = 0, 0
    while True:
        c = random.randint(1, 6)
        rolls += 1
        if prev == 5 and c == second:
            return rolls
        prev = c

def main():
    random.seed(12345)
    T = 100000
    e1 = sum(trial(6) for _ in range(T)) / T
    e2 = sum(trial(5) for _ in range(T)) / T
    print(f"Game 1 (five then six) expected rolls: {e1:.2f}")
    print(f"Game 2 (five then five) expected rolls: {e2:.2f}")
    print("Alice should play Game 1 (five then six), it has lower expected cost."
          if e1 < e2 else
          "Alice should play Game 2 (five then five), it has lower expected cost.")

if __name__ == "__main__":
    main()
