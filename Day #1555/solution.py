# Day 1555: Lazy binary tree: generate() returns a root in O(1) whose children are thunks
# (closures) forced on demand; a seeded coin flip (<1 continue prob) makes the tree
# finite w.p.1 but unbounded. Realization is depth-capped for a deterministic demo.


class LCG:
    def __init__(self, seed):
        self.s = seed

    def next(self):
        self.s = (self.s * 16807) % 2147483647  # Park-Miller
        return self.s

    def coin(self):
        return self.next() % 100 < 45  # child exists prob 0.45 -> finite


class Node:
    __slots__ = ("val", "left", "right")


_counter = [0]


def make_node(rng):
    # Does NOT force children: O(1).
    node = Node()
    node.val = _counter[0]
    _counter[0] += 1
    node.left = lambda: make_node(rng) if rng.coin() else None
    node.right = lambda: make_node(rng) if rng.coin() else None
    return node


def generate(rng):
    return make_node(rng)  # O(1): one node, children unevaluated


def realize(node, depth, cap):
    count = 1
    if depth < cap:
        l = node.left()
        if l is not None:
            count += realize(l, depth + 1, cap)
        r = node.right()
        if r is not None:
            count += realize(r, depth + 1, cap)
    return count


if __name__ == "__main__":
    rng = LCG(42)
    root = generate(rng)  # returns instantly
    print("generate() returned a lazy tree root in O(1)")
    n = realize(root, 0, 6)
    print("Realized tree node count: " + str(n))
