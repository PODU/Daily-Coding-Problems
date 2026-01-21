# Day 935: Check if a binary tree is height-balanced.
# Bottom-up DFS returning height, -1 signals imbalance. Time O(n), Space O(h).


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def is_balanced(root):
    def check(n):
        if n is None:
            return 0
        l = check(n.left)
        if l == -1:
            return -1
        r = check(n.right)
        if r == -1:
            return -1
        if abs(l - r) > 1:
            return -1
        return 1 + max(l, r)
    return check(root) != -1


if __name__ == "__main__":
    a = Node(1, Node(2, Node(4)), Node(3))
    print(is_balanced(a))  # True

    b = Node(1, Node(2, Node(3, Node(4))))
    print(is_balanced(b))  # False
