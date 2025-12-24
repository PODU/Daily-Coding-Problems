# Day 794: Max stack with O(1) push/pop/max using an auxiliary stack of running maxima.
# All operations O(1) time; O(n) space.


class MaxStack:
    def __init__(self):
        self._data = []
        self._maxs = []

    def push(self, val):
        self._data.append(val)
        if not self._maxs or val >= self._maxs[-1]:
            self._maxs.append(val)
        else:
            self._maxs.append(self._maxs[-1])

    def pop(self):
        if not self._data:
            return None
        self._maxs.pop()
        return self._data.pop()

    def max(self):
        return self._maxs[-1] if self._maxs else None


def main():
    s = MaxStack()
    for v in (1, 5, 3, 9, 2):
        s.push(v)
    print("max:", s.max())
    print("pop:", s.pop())
    print("pop:", s.pop())
    print("max:", s.max())


if __name__ == "__main__":
    main()
