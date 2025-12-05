# Day 696: 24-hour subscriber array with point update + inclusive range-sum query.
# Approach: Fenwick (Binary Indexed) Tree. update O(log n), query O(log n), space O(n).


class Fenwick:
    def __init__(self, n):
        self.n = n
        self.t = [0] * (n + 1)

    def add(self, i, v):
        i += 1
        while i <= self.n:
            self.t[i] += v
            i += i & -i

    def pref(self, i):
        i += 1
        s = 0
        while i > 0:
            s += self.t[i]
            i -= i & -i
        return s

    def range(self, l, r):
        return self.pref(r) - (self.pref(l - 1) if l else 0)


class Subscribers:
    def __init__(self):
        self.f = Fenwick(24)

    def update(self, hour, value):
        self.f.add(hour, value)

    def query(self, start, end):
        return self.f.range(start, end)


if __name__ == "__main__":
    s = Subscribers()
    s.update(3, 10); s.update(5, 7); s.update(10, 4)
    print(s.query(3, 10))  # 21
    print(s.query(0, 4))   # 10
    s.update(3, 5)
    print(s.query(3, 10))  # 26
