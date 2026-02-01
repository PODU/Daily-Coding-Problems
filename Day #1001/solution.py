# Day 1001: Validate a binary search tree.
# Recurse carrying an allowed (low, high) range; left key <= root <= right key.
# O(n) time, O(h) space.

class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def is_bst(node, low=float("-inf"), high=float("inf")):
    if node is None:
        return True
    if not (low <= node.val <= high):
        return False
    return is_bst(node.left, low, node.val) and is_bst(node.right, node.val, high)


if __name__ == "__main__":
    valid = Node(5, Node(3, Node(2), Node(4)), Node(8, Node(6), Node(9)))
    invalid = Node(5, Node(6), Node(8))  # left child 6 > 5
    print(is_bst(valid))    # True
    print(is_bst(invalid))  # False
