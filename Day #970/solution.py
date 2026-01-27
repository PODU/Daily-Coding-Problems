# Day 970: Space-efficient SparseArray storing only non-zero entries.
# Approach: dict of index->value, default 0. Time O(1) avg per op, Space O(#nonzero).


class SparseArray:
    def init(self, arr, size):
        self.size = size
        self.m = {i: v for i, v in enumerate(arr[:size]) if v != 0}

    def set(self, i, val):
        if not 0 <= i < self.size:
            raise IndexError(i)
        if val == 0:
            self.m.pop(i, None)
        else:
            self.m[i] = val

    def get(self, i):
        if not 0 <= i < self.size:
            raise IndexError(i)
        return self.m.get(i, 0)


if __name__ == '__main__':
    sa = SparseArray()
    sa.init([0, 0, 5, 0, 0, 0, 9, 0], 8)
    print(sa.get(2))  # 5
    print(sa.get(3))  # 0
    sa.set(3, 7)
    print(sa.get(3))  # 7
    sa.set(2, 0)
    print(sa.get(2))  # 0
