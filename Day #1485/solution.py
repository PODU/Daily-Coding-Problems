# Day 1485: Determine whether a k-ary tree is symmetric about its root.
# Two subtrees mirror iff their values match and child i of one mirrors child
# (k-1-i) of the other. Compare the root's children outside-in.
# Time O(N), Space O(H).

class Node:
    def __init__(self, val, children=None):
        self.val = val
        self.children = children or []


def is_mirror(a, b):
    if a.val != b.val or len(a.children) != len(b.children):
        return False
    k = len(a.children)
    return all(is_mirror(a.children[i], b.children[k - 1 - i]) for i in range(k))


def is_symmetric(root):
    if root is None:
        return True
    return is_mirror(root, root)


if __name__ == "__main__":
    tree = Node(4, [
        Node(3, [Node(9)]),
        Node(5),
        Node(3, [Node(9)]),
    ])
    print(is_symmetric(tree))  # True
