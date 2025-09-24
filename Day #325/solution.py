# Day 325: Unit conversion via weighted graph; addConversion adds a<->b edges, convert does BFS multiplying ratios.
# Time: O(V+E) per query, Space: O(V+E).
from collections import defaultdict, deque


class UnitConverter:
    def __init__(self):
        self.adj = defaultdict(list)

    def add_conversion(self, a, b, ratio):
        self.adj[a].append((b, ratio))
        self.adj[b].append((a, 1.0 / ratio))

    def convert(self, value, frm, to):
        if frm == to:
            return value
        dist = {frm: value}
        q = deque([frm])
        while q:
            u = q.popleft()
            for v, w in self.adj[u]:
                if v not in dist:
                    dist[v] = dist[u] * w
                    if v == to:
                        return dist[v]
                    q.append(v)
        return dist.get(to, float("nan"))


def main():
    uc = UnitConverter()
    uc.add_conversion("foot", "inch", 12)
    uc.add_conversion("yard", "foot", 3)
    uc.add_conversion("chain", "yard", 22)
    r = uc.convert(1, "yard", "inch")
    print("1 yard = {:.1f} inch".format(r))


if __name__ == "__main__":
    main()
