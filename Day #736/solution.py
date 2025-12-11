# Day 736: Count nodes in a complete binary tree.
# Compare left/right spine heights: full subtree -> 2^h-1, else recurse.
# Time: O(log^2 n), Space: O(log n).

class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def left_height(n):
    h = 0
    while n:
        h += 1
        n = n.left
    return h


def right_height(n):
    h = 0
    while n:
        h += 1
        n = n.right
    return h


def count_nodes(root):
    if not root:
        return 0
    lh, rh = left_height(root), right_height(root)
    if lh == rh:
        return (1 << lh) - 1
    return 1 + count_nodes(root.left) + count_nodes(root.right)


if __name__ == "__main__":
    root = Node(1, Node(2, Node(4), Node(5)), Node(3, Node(6)))
    print(count_nodes(root))  # 6
