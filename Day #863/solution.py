# Day 863: generate() returns a finite but arbitrarily large binary tree in O(1).
# Approach: root created in O(1); children expanded lazily with randomized termination
# (each child exists with prob < 0.5 => branching process is finite almost surely).
# generate(): O(1). Materializing whole tree: O(size). Deterministic demo via MINSTD RNG.

_state = [42]
P = 0.45
DEPTH_CAP = 50


def next_rand():
    _state[0] = (_state[0] * 48271) % 2147483647
    return _state[0] / 2147483647.0


class Node:
    __slots__ = ("left", "right", "expanded")

    def __init__(self):
        self.left = None
        self.right = None
        self.expanded = False


def ensure_children(n, depth):
    if n.expanded:
        return
    n.expanded = True
    if depth >= DEPTH_CAP:
        return
    if next_rand() < P:
        n.left = Node()
    if next_rand() < P:
        n.right = Node()


def generate():  # O(1)
    return Node()


def count_nodes(n, depth):
    if n is None:
        return 0
    ensure_children(n, depth)
    return 1 + count_nodes(n.left, depth + 1) + count_nodes(n.right, depth + 1)


if __name__ == "__main__":
    root = generate()
    print(f"Generated a finite binary tree with {count_nodes(root, 0)} "
          f"nodes (deterministic demo).")
