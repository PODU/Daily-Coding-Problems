# Day 1631: Validate BST via recursive inclusive min/max bounds (left<=root<=right). O(n) time, O(h) space.
import math


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def is_valid(n, lo, hi):
    if n is None:
        return True
    if n.val < lo or n.val > hi:
        return False
    return is_valid(n.left, lo, n.val) and is_valid(n.right, n.val, hi)


def validate(root):
    return is_valid(root, -math.inf, math.inf)


def main():
    # Valid BST: root 5, left 3 (2,4), right 8 (6,9)
    root = Node(5)
    root.left = Node(3)
    root.left.left = Node(2)
    root.left.right = Node(4)
    root.right = Node(8)
    root.right.left = Node(6)
    root.right.right = Node(9)

    # Invalid: root 5, left child 6
    bad = Node(5)
    bad.left = Node(6)

    print("true" if validate(root) else "false")
    print("true" if validate(bad) else "false")


if __name__ == "__main__":
    main()
