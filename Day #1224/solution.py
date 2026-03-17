# Day 1224: DFS with BST pruning: skip left if val<a, skip right if val>b. O(n) worst.
# O(n) time worst, O(h) space (recursion).
from typing import Optional


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def range_sum_bst(node: Optional[Node], a: int, b: int) -> int:
    if not node:
        return 0
    if node.val < a:
        return range_sum_bst(node.right, a, b)
    if node.val > b:
        return range_sum_bst(node.left, a, b)
    return node.val + range_sum_bst(node.left, a, b) + range_sum_bst(node.right, a, b)


if __name__ == "__main__":
    root = Node(5, Node(3, Node(2), Node(4)), Node(8, Node(6), Node(10)))
    print(range_sum_bst(root, 4, 9))
