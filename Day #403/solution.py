# Day 403: rand7 from rand5 via rejection sampling: idx=(rand5-1)*5+rand5 in 1..25, reject>21,
# return (idx-1)%7+1. O(1) expected calls. Space O(1).
import random

rng = random.Random(12345)


def rand5():
    return rng.randint(1, 5)


def rand7():
    while True:
        idx = (rand5() - 1) * 5 + rand5()  # uniform 1..25
        if idx <= 21:
            return (idx - 1) % 7 + 1


def main():
    N = 70000
    counts = [0] * 8
    for _ in range(N):
        counts[rand7()] += 1
    for v in range(1, 8):
        print(f"{v}: {counts[v]} " + "#" * (counts[v] // 250))


if __name__ == "__main__":
    main()
