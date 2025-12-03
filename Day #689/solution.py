# Day 689: Unit conversion as a weighted graph; convert via BFS multiplying edge ratios.
# add_conversion(a, b, r): 1 a = r b. Time O(V + E) per query, Space O(V + E).
from collections import defaultdict, deque


class UnitConverter:
    def __init__(self):
        self.graph = defaultdict(dict)

    def add_conversion(self, frm, to, factor):
        # 1 frm = factor to
        self.graph[frm][to] = factor
        self.graph[to][frm] = 1.0 / factor

    def convert(self, qty, frm, to):
        if frm == to:
            return qty
        if frm not in self.graph or to not in self.graph:
            return None
        visited = {frm}
        queue = deque([(frm, 1.0)])
        while queue:
            unit, ratio = queue.popleft()
            if unit == to:
                return qty * ratio
            for nxt, f in self.graph[unit].items():
                if nxt not in visited:
                    visited.add(nxt)
                    queue.append((nxt, ratio * f))
        return None


def main():
    uc = UnitConverter()
    uc.add_conversion("foot", "inch", 12)    # 12 inches = 1 foot
    uc.add_conversion("yard", "foot", 3)     # 3 feet = 1 yard
    uc.add_conversion("chain", "yard", 22)   # 22 yards = 1 chain

    print("1 chain = %d inches" % round(uc.convert(1, "chain", "inch")))
    print("1 yard = %d inches" % round(uc.convert(1, "yard", "inch")))


if __name__ == "__main__":
    main()
