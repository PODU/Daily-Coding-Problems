# Day 1529: Prune binary tree: remove subtrees containing only 0s (no 1 descendant).
# Post-order recursion. O(n) time, O(h) recursion stack.
from collections import deque


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def prune(root):
    if root is None:
        return None
    root.left = prune(root.left)
    root.right = prune(root.right)
    if root.val == 0 and root.left is None and root.right is None:
        return None
    return root


def level_order(root):
    q = deque([root])
    out = []
    while q:
        n = q.popleft()
        if n is None:
            out.append("null")
            continue
        out.append(str(n.val))
        q.append(n.left)
        q.append(n.right)
    while out and out[-1] == "null":
        out.pop()
    return "[" + ", ".join(out) + "]"


if __name__ == "__main__":
    root = Node(0,
                Node(1),
                Node(0,
                     Node(1, Node(0), Node(0)),
                     Node(0)))
    root = prune(root)
    print(level_order(root))  # [0, 1, 0, null, null, 1]
