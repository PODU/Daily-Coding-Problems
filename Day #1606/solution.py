# Day 1606: Check if a binary tree is height-balanced.
# Bottom-up recursion returning height, -1 if unbalanced. Time O(n), Space O(h).

class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def is_balanced(root):
    def check(node):  # returns height, or -1 if unbalanced
        if not node:
            return 0
        l = check(node.left)
        if l == -1:
            return -1
        r = check(node.right)
        if r == -1:
            return -1
        if abs(l - r) > 1:
            return -1
        return max(l, r) + 1

    return check(root) != -1


if __name__ == "__main__":
    root = Node(1, Node(2, Node(4)), Node(3))
    print("true" if is_balanced(root) else "false")  # true
