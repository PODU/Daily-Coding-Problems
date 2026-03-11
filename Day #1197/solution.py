# Day 1197: Max stack with O(1) push/pop/max.
# Auxiliary stack stores running maxima alongside main stack. All ops O(1); space O(N).

class MaxStack:
    def __init__(self):
        self._data = []
        self._maxs = []

    def push(self, val):
        self._data.append(val)
        self._maxs.append(val if not self._maxs else max(val, self._maxs[-1]))

    def pop(self):
        # returns top, or None if empty
        if not self._data:
            return None
        self._maxs.pop()
        return self._data.pop()

    def max(self):
        return None if not self._maxs else self._maxs[-1]


def show(v):
    return "null" if v is None else str(v)


if __name__ == "__main__":
    s = MaxStack()
    for x in (3, 1, 5, 2):
        s.push(x)
    print("max:", show(s.max()))
    print("pop:", show(s.pop()))
    print("pop:", show(s.pop()))
    print("max:", show(s.max()))
