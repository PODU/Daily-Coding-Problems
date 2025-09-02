# Day 204: Count nodes of a complete binary tree faster than O(n).
# Compare left/right spine heights: if equal subtree is perfect (2^h - 1), else recurse.
# Time: O(log^2 n), Space: O(log n).


class Node:
    def __init__(self):
        self.left = None
        self.right = None


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
        return (1 << lh) - 1  # perfect subtree
    return 1 + count_nodes(root.left) + count_nodes(root.right)


if __name__ == "__main__":
    n = [None] + [Node() for _ in range(6)]
    n[1].left, n[1].right = n[2], n[3]
    n[2].left, n[2].right = n[4], n[5]
    n[3].left = n[6]
    print(count_nodes(n[1]))  # 6
