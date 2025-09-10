# Day 247: Height-balanced binary tree check.
# Bottom-up DFS returning subtree height, or -1 if unbalanced. O(n) time, O(h) space.

class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def check(root):
    if root is None:
        return 0
    l = check(root.left)
    if l == -1:
        return -1
    r = check(root.right)
    if r == -1:
        return -1
    if abs(l - r) > 1:
        return -1
    return max(l, r) + 1


def is_balanced(root):
    return check(root) != -1


def main():
    a = Node(1)
    a.left = Node(2)
    a.right = Node(3)
    a.left.left = Node(4)

    b = Node(1)
    b.left = Node(2)
    b.left.left = Node(3)

    print("Balanced tree:", "true" if is_balanced(a) else "false")
    print("Unbalanced tree:", "true" if is_balanced(b) else "false")


if __name__ == "__main__":
    main()
