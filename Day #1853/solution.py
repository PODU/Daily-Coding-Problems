# Day 1853: Stack with O(1) push/pop/max.
# Keep an auxiliary stack of running maxima alongside the data stack. All ops O(1) time, O(n) space.


class MaxStack:
    def __init__(self):
        self._data = []
        self._maxs = []

    def push(self, v):
        self._data.append(v)
        self._maxs.append(v if not self._maxs else max(self._maxs[-1], v))

    def pop(self):
        if not self._data:
            raise IndexError("empty stack")
        self._maxs.pop()
        return self._data.pop()

    def max(self):
        if not self._maxs:
            raise IndexError("empty stack")
        return self._maxs[-1]


if __name__ == "__main__":
    s = MaxStack()
    s.push(1); s.push(5); s.push(3)
    print(s.max())  # 5
    print(s.pop())  # 3
    print(s.max())  # 5
    print(s.pop())  # 5
    print(s.max())  # 1
