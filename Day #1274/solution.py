# Day 1274: Prune a 0/1 binary tree, removing every subtree that contains only 0s.
# Post-order recursion: a node survives iff it is 1 or has a surviving child. O(n).
from typing import Optional


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def prune(node: Optional[Node]) -> Optional[Node]:
    if node is None:
        return None
    node.left = prune(node.left)
    node.right = prune(node.right)
    if node.val == 0 and node.left is None and node.right is None:
        return None
    return node


def serialize(node: Optional[Node]) -> str:
    if node is None:
        return "null"
    return f"{node.val}({serialize(node.left)},{serialize(node.right)})"


if __name__ == "__main__":
    root = Node(0, Node(1), Node(0, Node(1, Node(0), Node(0)), Node(0)))
    root = prune(root)
    # Pruned tree: 0(1(null,null),0(1(null,null),null))
    print(serialize(root))
