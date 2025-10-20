# Day 466: Symmetric k-ary tree: recursively compare children of two subtrees in mirror order.
# Time: O(n), Space: O(h) recursion.
class Node:
    def __init__(self, val):
        self.val = val
        self.children = []


def mirror(a, b):
    if a is None and b is None:
        return True
    if a is None or b is None:
        return False
    if a.val != b.val:
        return False
    if len(a.children) != len(b.children):
        return False
    n = len(a.children)
    return all(mirror(a.children[i], b.children[n - 1 - i]) for i in range(n))


def is_symmetric(root):
    if root is None:
        return True
    return mirror(root, root)


if __name__ == "__main__":
    root = Node(4)
    l3, m5, r3 = Node(3), Node(5), Node(3)
    root.children = [l3, m5, r3]
    l3.children = [Node(9)]
    r3.children = [Node(9)]
    print("True" if is_symmetric(root) else "False")
