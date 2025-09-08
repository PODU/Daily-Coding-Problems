# Day 237: Symmetric k-ary tree: a tree is symmetric iff left subtree mirrors right subtree.
# Recursively compare children in mirrored order. Time: O(N), Space: O(height).


class Node:
    def __init__(self, val, children=None):
        self.val = val
        self.children = children or []


def mirror(a, b):
    if a is None and b is None:
        return True
    if a is None or b is None:
        return False
    if a.val != b.val or len(a.children) != len(b.children):
        return False
    n = len(a.children)
    return all(mirror(a.children[i], b.children[n - 1 - i]) for i in range(n))


def is_symmetric(root):
    if root is None:
        return True
    n = len(root.children)
    return all(mirror(root.children[i], root.children[n - 1 - i]) for i in range(n // 2))


if __name__ == "__main__":
    root = Node(4, [
        Node(3, [Node(9)]),
        Node(5),
        Node(3, [Node(9)]),
    ])
    print(is_symmetric(root))  # True
