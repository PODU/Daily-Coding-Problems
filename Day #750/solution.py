# Day 750: generate() returns a root in O(1); children are materialized lazily on first access.
# Each child exists with probability p<0.5, so the tree is finite (a.s.) yet unbounded.
# generate(): O(1). Traversal materializes nodes on demand.
import random

P = 0.45


class LazyNode:
    def __init__(self, value, rng):
        self.value = value
        self.rng = rng
        self._left = None
        self._right = None
        self._lset = False
        self._rset = False

    def left(self):
        if not self._lset:
            self._lset = True
            self._left = LazyNode(0, self.rng) if self.rng.random() < P else None
        return self._left

    def right(self):
        if not self._rset:
            self._rset = True
            self._right = LazyNode(0, self.rng) if self.rng.random() < P else None
        return self._right


def generate(rng):
    return LazyNode(0, rng)  # O(1)


def tree_size(node):
    if node is None:
        return 0
    return 1 + tree_size(node.left()) + tree_size(node.right())


if __name__ == "__main__":
    rng = random.Random(42)
    root = generate(rng)  # O(1)
    print("Generated finite tree size:", tree_size(root))
