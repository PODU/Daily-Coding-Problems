# Day 83: Invert (mirror) a binary tree by swapping children recursively.
# Time O(n), Space O(h).
from collections import deque


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def invert(root):
    if root is None:
        return None
    root.left, root.right = root.right, root.left
    invert(root.left)
    invert(root.right)
    return root


def level_order(root):
    out, q = [], deque([root] if root else [])
    while q:
        n = q.popleft()
        out.append(n.val)
        if n.left:
            q.append(n.left)
        if n.right:
            q.append(n.right)
    return " ".join(out)


if __name__ == "__main__":
    a = Node("a", Node("b", Node("d"), Node("e")), Node("c", Node("f")))
    print("before:", level_order(a))  # a b c d e f
    invert(a)
    print("after: ", level_order(a))  # a c b f e d
