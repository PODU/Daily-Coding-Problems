# Day 45: rand7 from rand5: rejection sampling over 5*(rand5-1)+rand5 in 1..25,
# reject >21, map ((v-1)%7)+1. Expected O(1) amortized. rand5 from a seeded LCG.

_state = [1]  # deterministic seed


def rand5():
    _state[0] = (_state[0] * 75 + 74) % 65537
    return _state[0] % 5 + 1  # uniform-ish 1..5 for the demo


def rand7():
    while True:
        v = 5 * (rand5() - 1) + rand5()  # 1..25
        if v <= 21:
            return (v - 1) % 7 + 1


if __name__ == "__main__":
    samples = [rand7() for _ in range(20)]
    print("rand7 samples: " + " ".join(str(x) for x in samples))

    counts = [0] * 8
    for _ in range(7000):
        counts[rand7()] += 1
    print("counts over 7000 trials: " + " ".join(f"{v}:{counts[v]}" for v in range(1, 8)))
