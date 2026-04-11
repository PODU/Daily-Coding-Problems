# Day 1341: Check height-balanced binary tree via bottom-up DFS; -1 sentinel marks unbalanced.
# Time O(n), Space O(h).

class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def height(root):
    if root is None:
        return 0
    l = height(root.left)
    if l == -1:
        return -1
    r = height(root.right)
    if r == -1:
        return -1
    if abs(l - r) > 1:
        return -1
    return max(l, r) + 1


def is_balanced(root):
    return height(root) != -1


if __name__ == "__main__":
    # Balanced tree [1,2,3,4,5,null,6]
    a = Node(1, Node(2, Node(4), Node(5)), Node(3, None, Node(6)))
    print(f"Balanced: {str(is_balanced(a)).lower()}")

    # Skewed tree 1 -> 2 -> 3
    b = Node(1, Node(2, Node(3)))
    print(f"Balanced: {str(is_balanced(b)).lower()}")
