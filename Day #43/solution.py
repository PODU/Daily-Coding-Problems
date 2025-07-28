# Day 43: Max Stack: main stack + auxiliary stack holding running max. All ops O(1).
# Time O(1) per op; Space O(n).

class MaxStack:
    def __init__(self):
        self._data = []
        self._maxes = []

    def push(self, v):
        self._data.append(v)
        self._maxes.append(v if not self._maxes else max(v, self._maxes[-1]))

    def pop(self):
        if not self._data:
            return None
        self._maxes.pop()
        return self._data.pop()

    def max(self):
        return None if not self._maxes else self._maxes[-1]


if __name__ == "__main__":
    s = MaxStack()
    for v in [3, 1, 5, 4]:
        s.push(v)
        print(f"push {v} -> max={s.max()}")
    print(f"pop -> {s.pop()}, max={s.max()}")
    print(f"pop -> {s.pop()}, max={s.max()}")
