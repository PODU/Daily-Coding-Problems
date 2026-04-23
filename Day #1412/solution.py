# Day 1412: generate() returns a finite but arbitrarily large binary tree in O(1).
# Approach: lazy nodes — children are materialized only on access (here via a deterministic
# LCG so the demo is reproducible). generate() itself is O(1); a child appears on first touch.

_lcg = [42]


def next_rand():
    _lcg[0] = (_lcg[0] * 1103515245 + 12345) & 0xFFFFFFFFFFFFFFFF
    return (_lcg[0] >> 16) & 0x7FFF


class LazyNode:
    def __init__(self, depth):
        self.depth = depth
        self._left_done = False
        self._right_done = False
        self._left = None
        self._right = None

    def _spawn(self):
        bound = 55 - self.depth * 7
        return bound > 0 and (next_rand() % 100) < bound

    def left(self):
        if not self._left_done:
            self._left_done = True
            if self._spawn():
                self._left = LazyNode(self.depth + 1)
        return self._left

    def right(self):
        if not self._right_done:
            self._right_done = True
            if self._spawn():
                self._right = LazyNode(self.depth + 1)
        return self._right


def generate():
    # O(1): just return a root; the tree unfolds lazily on access.
    return LazyNode(0)


def count_nodes(n):
    if n is None:
        return 0
    l = count_nodes(n.left())
    r = count_nodes(n.right())
    return 1 + l + r


if __name__ == "__main__":
    root = generate()
    print("Tree size:", count_nodes(root))
