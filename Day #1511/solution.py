# Day 1511: Range Sum of BST via DFS with BST pruning (skip left if node<a, right if node>b).
# O(n) worst-case time, O(h) recursion space.

class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def range_sum(root, a, b):
    if root is None:
        return 0
    if root.val < a:
        return range_sum(root.right, a, b)
    if root.val > b:
        return range_sum(root.left, a, b)
    return root.val + range_sum(root.left, a, b) + range_sum(root.right, a, b)


if __name__ == "__main__":
    root = Node(5)
    root.left = Node(3)
    root.right = Node(8)
    root.left.left = Node(2)
    root.left.right = Node(4)
    root.right.left = Node(6)
    root.right.right = Node(10)
    print(range_sum(root, 4, 9))
