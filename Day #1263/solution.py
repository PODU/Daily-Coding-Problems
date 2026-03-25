# Day 1263: All root-to-leaf paths in a binary tree.
# DFS carrying the current path. O(n) nodes visited, O(h) recursion + output size.
from typing import List, Optional


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def root_to_leaf(root: Optional[Node]) -> List[List[int]]:
    res = []

    def dfs(node, path):
        if not node:
            return
        path.append(node.val)
        if not node.left and not node.right:
            res.append(list(path))
        else:
            dfs(node.left, path)
            dfs(node.right, path)
        path.pop()

    dfs(root, [])
    return res


if __name__ == "__main__":
    root = Node(1, Node(2), Node(3, Node(4), Node(5)))
    print(root_to_leaf(root))
