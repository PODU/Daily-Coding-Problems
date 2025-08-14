# Day 116: generate() returns a root in O(1); children materialize lazily on access.
# Each node spawns children with a depth-decaying probability => finite a.s. but unbounded.
# (Demo uses a seeded Park-Miller LCG so the forced size is reproducible.)
class Node:
    def __init__(self, depth):
        self.depth = depth
        self.l = None
        self.r = None


_lcg = [42]


def next_rand():
    _lcg[0] = (_lcg[0] * 16807) % 2147483647
    return _lcg[0]


def threshold(d):
    return max(0, 750 - 80 * d)


def generate():
    return Node(0)  # O(1)


def force(n):
    cnt = 1
    if next_rand() % 1000 < threshold(n.depth):
        n.l = Node(n.depth + 1)
        cnt += force(n.l)
    if next_rand() % 1000 < threshold(n.depth):
        n.r = Node(n.depth + 1)
        cnt += force(n.r)
    return cnt


if __name__ == "__main__":
    root = generate()
    n = force(root)
    print("Generated a finite binary tree with %d nodes" % n)
