# Day 1445: Unit converter. Model units as a weighted graph (edge = conversion
# factor) and BFS for a path on query. add_unit O(1); convert O(V+E).
from collections import deque, defaultdict
import math


class UnitConverter:
    def __init__(self):
        self.graph = defaultdict(dict)  # graph[a][b] = factor; 1 a = factor b

    def add_unit(self, frm, to, factor):
        self.graph[frm][to] = factor
        self.graph[to][frm] = 1.0 / factor

    def convert(self, value, frm, to):
        if frm == to:
            return value
        dist = {frm: 1.0}
        q = deque([frm])
        while q:
            u = q.popleft()
            for v, f in self.graph[u].items():
                if v not in dist:
                    dist[v] = dist[u] * f
                    if v == to:
                        return value * dist[v]
                    q.append(v)
        return math.nan


if __name__ == "__main__":
    uc = UnitConverter()
    uc.add_unit("foot", "inch", 12)
    uc.add_unit("yard", "foot", 3)
    uc.add_unit("chain", "yard", 22)
    print(int(uc.convert(1, "yard", "inch")))  # 36
