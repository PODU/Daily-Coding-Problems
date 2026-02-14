# Day 1064: Implement rand7() from rand5() via rejection sampling.
# (rand5()-1)*5 + rand5() -> uniform 1..25; reject >21; ((v-1)%7)+1. Expected O(1) calls, O(1) space.
import random

random.seed(42)


def rand5():
    return random.randint(1, 5)


def rand7():
    while True:
        v = (rand5() - 1) * 5 + rand5()  # uniform 1..25
        if v <= 21:
            return ((v - 1) % 7) + 1


if __name__ == "__main__":
    counts = [0] * 8
    for _ in range(70000):
        counts[rand7()] += 1
    for i in range(1, 8):
        print(f"{i}: {counts[i]}")
