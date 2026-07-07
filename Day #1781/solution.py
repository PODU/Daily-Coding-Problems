# Day 1781: Unit converter as a graph: add_relation stores 1 a = factor b (edge a->b, b->a=1/factor).
# convert does BFS multiplying factors along the path. Time O(V+E) per query, Space O(V+E).
from collections import deque, defaultdict


class UnitConverter:
    def __init__(self):
        self.adj = defaultdict(dict)

    def add_relation(self, a, b, factor):
        self.adj[a][b] = factor
        self.adj[b][a] = 1.0 / factor

    def convert(self, qty, frm, to):
        if frm == to:
            return qty
        if frm not in self.adj or to not in self.adj:
            return None
        dist = {frm: qty}
        q = deque([frm])
        while q:
            u = q.popleft()
            for nxt, f in self.adj[u].items():
                if nxt not in dist:
                    dist[nxt] = dist[u] * f
                    if nxt == to:
                        return dist[nxt]
                    q.append(nxt)
        return None


def main():
    uc = UnitConverter()
    uc.add_relation("inches", "foot", 1.0 / 12.0)
    uc.add_relation("feet", "yard", 1.0 / 3.0)
    uc.add_relation("yards", "chain", 1.0 / 22.0)
    uc.add_relation("foot", "feet", 1.0)
    uc.add_relation("yard", "yards", 1.0)

    print(f"1 yard = {uc.convert(1, 'yard', 'inches'):.0f} inches")
    print(f"1 chain = {uc.convert(1, 'chain', 'inches'):.0f} inches")
    print(f"1 chain = {uc.convert(1, 'chain', 'feet'):.0f} feet")


if __name__ == "__main__":
    main()
