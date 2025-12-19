# Day 761: rand5() from rand7() via rejection sampling.
# Accept values 1..5, reject 6..7 and retry. Uniform; expected O(1) calls (7/5).

_state = [1]


def rand7():
    _state[0] = (_state[0] * 1103515245 + 12345) & 0x7fffffff
    return _state[0] % 7 + 1   # uniform 1..7


def rand5():
    while True:
        x = rand7()
        if x <= 5:
            return x


if __name__ == "__main__":
    N = 100000
    cnt = [0] * 6
    for _ in range(N):
        cnt[rand5()] += 1
    print(f"counts over {N} samples:")
    for v in range(1, 6):
        print(f"{v}: {cnt[v]}")
