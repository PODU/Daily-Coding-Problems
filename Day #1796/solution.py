# Day 1796: Uniform random in [0,n-1] excluding values in l: precompute sorted allowed array, pick random index. O(n) build, O(allowed) space.
import random


class RandomPicker:
    def __init__(self, n, l):
        ex = set(l)
        self.allowed = [i for i in range(n) if i not in ex]

    def pick(self):
        return random.choice(self.allowed)


if __name__ == "__main__":
    random.seed(12345)
    rp = RandomPicker(10, [2, 3, 5, 7])
    aset = set(rp.allowed)
    print("allowed=" + str(rp.allowed))
    ok = all(rp.pick() in aset for _ in range(1000))
    print("sample in allowed:", "true" if ok else "false")
