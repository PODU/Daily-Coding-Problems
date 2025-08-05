# Day 71: rand5 from rand7 via rejection sampling: call rand7() until <=5. Uniform 1..5. Time O(1) expected, Space O(1).
import random


def rand7():
    return random.randint(1, 7)


def rand5():
    while True:
        x = rand7()
        if x <= 5:
            return x


def main():
    trials = 100000
    counts = [0] * 6
    for _ in range(trials):
        v = rand5()
        assert 1 <= v <= 5
        counts[v] += 1
    expected = trials / 5.0
    for v in range(1, 6):
        assert abs(counts[v] - expected) < expected * 0.1
    print("rand5 OK")


if __name__ == "__main__":
    main()
