# Day 587: Binary tree root-to-leaf paths.
# Approach: DFS, accumulate current path, record at leaves. Time O(n), Space O(h).
from typing import List, Optional


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def root_to_leaf_paths(root: Optional[Node]) -> List[List[int]]:
    res: List[List[int]] = []

    def dfs(node: Optional[Node], path: List[int]) -> None:
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
    print(root_to_leaf_paths(root))
