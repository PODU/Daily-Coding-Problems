# Day 1010: Unit Converter: graph where edge a->b stores factor (1 a = f b).
# convert() does BFS multiplying factors. Time O(V+E) per query, Space O(V+E).
from collections import defaultdict, deque
import math


class UnitConverter:
    def __init__(self):
        self.g = defaultdict(list)

    def addUnit(self, frm, to, factor):
        self.g[frm].append((to, factor))
        self.g[to].append((frm, 1.0 / factor))

    def convert(self, value, frm, to):
        if frm == to:
            return value
        dist = {frm: 1.0}
        q = deque([frm])
        while q:
            u = q.popleft()
            for v, f in self.g[u]:
                if v not in dist:
                    dist[v] = dist[u] * f
                    if v == to:
                        return value * dist[v]
                    q.append(v)
        return math.nan


def main():
    uc = UnitConverter()
    uc.addUnit("inch", "foot", 1.0 / 12)
    uc.addUnit("foot", "yard", 1.0 / 3)
    uc.addUnit("yard", "chain", 1.0 / 22)

    print(f"1 chain = {round(uc.convert(1, 'chain', 'inch'))} inches")
    print(f"36 inches = {round(uc.convert(36, 'inch', 'yard'))} yards")


if __name__ == "__main__":
    main()
