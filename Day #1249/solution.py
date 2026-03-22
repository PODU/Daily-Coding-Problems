# Day 1249: Validate BST with inclusive bounds (left <= node <= right) via recursive range check. O(n) time, O(h) space.


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def is_valid(node, lo, hi):
    if node is None:
        return True
    if node.val < lo or node.val > hi:
        return False
    return is_valid(node.left, lo, node.val) and is_valid(node.right, node.val, hi)


def is_valid_bst(root):
    return is_valid(root, float("-inf"), float("inf"))


def main():
    # Valid tree: root 5, left 3 (2,5), right 8 (5,12)
    root = Node(5)
    root.left = Node(3)
    root.left.left = Node(2)
    root.left.right = Node(5)
    root.right = Node(8)
    root.right.left = Node(5)
    root.right.right = Node(12)
    print("true" if is_valid_bst(root) else "false")

    # Invalid tree: root 5, left 6 (6 > 5 violates)
    bad = Node(5)
    bad.left = Node(6)
    bad.right = Node(8)
    print("true" if is_valid_bst(bad) else "false")


if __name__ == "__main__":
    main()
