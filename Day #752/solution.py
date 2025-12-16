# Day 752: Level-order (BFS) traversal of a binary tree.
# Time: O(n), Space: O(w) where w is the max width of the tree.
from collections import deque


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def level_order(root):
    out = []
    if not root:
        return out
    q = deque([root])
    while q:
        n = q.popleft()
        out.append(n.val)
        if n.left:
            q.append(n.left)
        if n.right:
            q.append(n.right)
    return out


if __name__ == "__main__":
    #   1
    #  / \
    # 2   3
    #    / \
    #   4   5
    root = Node(1, Node(2), Node(3, Node(4), Node(5)))
    print(", ".join(map(str, level_order(root))))
