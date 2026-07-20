# Day 1849: SparseArray storing only non-zero entries in a dict.
# init O(N) once, set/get O(1) average. Space O(non-zero count).


class SparseArray:
    def __init__(self, arr, size):
        self.size = size
        self.data = {i: v for i, v in enumerate(arr) if v != 0}

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


def main():
    sa = SparseArray([0, 0, 5, 0, 0, 0, 9, 0], 8)
    print(sa.get(2))  # 5
    print(sa.get(3))  # 0
    sa.set(3, 7)
    print(sa.get(3))  # 7
    sa.set(2, 0)
    print(sa.get(2))  # 0


if __name__ == "__main__":
    main()
