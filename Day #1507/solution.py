# Day 1507: Reconstruct BST from postorder. Process postorder from the right with an
# upper-bound recursion (reverse postorder = root,right,left). Time O(n), Space O(h).
import sys


class Node:
    __slots__ = ("val", "left", "right")
    def __init__(self, v):
        self.val = v
        self.left = None
        self.right = None


def bst_from_postorder(post):
    idx = [len(post) - 1]

    def build(bound):
        if idx[0] < 0 or post[idx[0]] < bound:
            return None
        root = Node(post[idx[0]])
        idx[0] -= 1
        root.right = build(root.val)
        root.left = build(bound)
        return root

    return build(float("-inf"))


def preorder(root, out):
    if root is None:
        return
    out.append(root.val)
    preorder(root.left, out)
    preorder(root.right, out)


if __name__ == "__main__":
    sys.setrecursionlimit(10000)
    post = [2, 4, 3, 8, 7, 5]
    root = bst_from_postorder(post)
    pre = []
    preorder(root, pre)
    print(" ".join(str(x) for x in pre))
