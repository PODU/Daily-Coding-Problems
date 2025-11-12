# Day 588: Space-efficient SparseArray backed by a dict of non-zero entries.
# Approach: store only non-zero indices. Time O(1) avg per op, Space O(#nonzero).
from typing import List


class SparseArray:
    def __init__(self, arr: List[int] = None, size: int = 0):
        self.data = {}
        self.size = 0
        if arr is not None:
            self.init(arr, size)

    def init(self, arr: List[int], size: int) -> None:
        self.size = size
        self.data = {i: v for i, v in enumerate(arr[:size]) if v != 0}

    def set(self, i: int, val: int) -> None:
        if not 0 <= i < self.size:
            raise IndexError("index out of range")
        if val == 0:
            self.data.pop(i, None)
        else:
            self.data[i] = val

    def get(self, i: int) -> int:
        if not 0 <= i < self.size:
            raise IndexError("index out of range")
        return self.data.get(i, 0)


if __name__ == "__main__":
    sa = SparseArray()
    sa.init([0, 0, 0, 5, 0, 0, 9, 0], 8)
    print("get(3) =", sa.get(3))  # 5
    print("get(6) =", sa.get(6))  # 9
    print("get(0) =", sa.get(0))  # 0
    sa.set(1, 42)
    print("after set(1,42), get(1) =", sa.get(1))  # 42
    sa.set(3, 0)
    print("after set(3,0), get(3) =", sa.get(3))  # 0
