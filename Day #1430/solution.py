# Day 1430: Space-efficient SparseArray for a mostly-zero array.
# Approach: store only non-zero indices in a dict. Time: O(1) avg per op, Space: O(#nonzero).


class SparseArray:
    def __init__(self):
        self._data = {}
        self._n = 0

    def init(self, arr, size):
        self._n = size
        self._data = {i: v for i, v in enumerate(arr[:size]) if v != 0}

    def set(self, i, val):
        if not 0 <= i < self._n:
            raise IndexError("index out of bounds")
        if val == 0:
            self._data.pop(i, None)
        else:
            self._data[i] = val

    def get(self, i):
        if not 0 <= i < self._n:
            raise IndexError("index out of bounds")
        return self._data.get(i, 0)


if __name__ == "__main__":
    sa = SparseArray()
    sa.init([0, 0, 5, 0, 0, 0, 9, 0], 8)
    print(sa.get(2))  # 5
    print(sa.get(3))  # 0
    sa.set(3, 7)
    print(sa.get(3))  # 7
    sa.set(2, 0)
    print(sa.get(2))  # 0
