# Day 134: SparseArray storing only non-zero entries in a hash map.
# init O(n) once, set/get O(1) average. Space O(#nonzero).


class SparseArray:
    def __init__(self):
        self.data = {}
        self.size = 0

    def init(self, arr, sz):
        self.size = sz
        self.data = {i: v for i, v in enumerate(arr[:sz]) if v != 0}

    def set(self, i, val):
        if not 0 <= i < self.size:
            raise IndexError(i)
        if val == 0:
            self.data.pop(i, None)
        else:
            self.data[i] = val

    def get(self, i):
        if not 0 <= i < self.size:
            raise IndexError(i)
        return self.data.get(i, 0)


if __name__ == "__main__":
    sa = SparseArray()
    sa.init([0, 0, 7, 0, 0, 0, 3, 0], 8)
    print(sa.get(2))  # 7
    print(sa.get(0))  # 0
    sa.set(0, 5)
    print(sa.get(0))  # 5
    sa.set(2, 0)
    print(sa.get(2))  # 0
