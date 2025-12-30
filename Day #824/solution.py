# Day 824: Merge two binary trees recursively; node value = sum, missing nodes taken from the other.
# Time: O(n), Space: O(h) recursion.

class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def merge(a, b):
    if a is None:
        return b
    if b is None:
        return a
    n = Node(a.val + b.val)
    n.left = merge(a.left, b.left)
    n.right = merge(a.right, b.right)
    return n


def preorder(n, out):
    if n is None:
        return
    out.append(n.val)
    preorder(n.left, out)
    preorder(n.right, out)


if __name__ == "__main__":
    t1 = Node(1, Node(3, Node(5)), Node(2))
    t2 = Node(2, Node(1, None, Node(4)), Node(3, None, Node(7)))
    m = merge(t1, t2)
    res = []
    preorder(m, res)
    print(res)
