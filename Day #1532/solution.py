# Day 1532: Invert (mirror) a binary tree by swapping left/right children of every node.
# Time O(n), Space O(h) recursion.
class Node:
    def __init__(self, val, l=None, r=None):
        self.val = val
        self.l = l
        self.r = r


def invert(root):
    if root is None:
        return None
    root.l, root.r = root.r, root.l
    invert(root.l)
    invert(root.r)
    return root


def preorder(root, out):
    if root is None:
        return
    out.append(root.val)
    preorder(root.l, out)
    preorder(root.r, out)


if __name__ == "__main__":
    a = Node('a', Node('b', Node('d'), Node('e')), Node('c', Node('f')))
    before = []
    preorder(a, before)
    invert(a)
    after = []
    preorder(a, after)
    print("before (preorder):", " ".join(before))
    print("after  (preorder):", " ".join(after))
