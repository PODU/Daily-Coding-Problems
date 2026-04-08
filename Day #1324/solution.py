# Day 1324: Point-update / range-sum over a 24-element array using a Fenwick (Binary Indexed) Tree.
# update O(log n), query O(log n). 1-indexed internally over fixed size 24.

class Subscribers:
    def __init__(self):
        self.tree = [0] * 25

    def update(self, hour, value):
        i = hour + 1
        while i <= 24:
            self.tree[i] += value
            i += i & (-i)

    def _prefix(self, hour):
        s, i = 0, hour + 1
        while i > 0:
            s += self.tree[i]
            i -= i & (-i)
        return s

    def query(self, start, end):
        return self._prefix(end) - (self._prefix(start - 1) if start > 0 else 0)


if __name__ == "__main__":
    s = Subscribers()
    s.update(2, 10)
    s.update(5, 3)
    s.update(5, 7)
    print(s.query(2, 5))   # 20
    print(s.query(0, 23))  # 20
    print(s.query(3, 4))   # 0
