# Day 93: Largest BST subtree size. Post-order DFS returning
# (is_bst, size, min, max) per node, combining children in O(n) time, O(h) space.
import math


class Node:
    def __init__(self, val, left=None, right=None):
        self.val, self.left, self.right = val, left, right


def largest_bst(root):
    best = 0

    def dfs(node):
        nonlocal best
        if not node:
            return (True, 0, math.inf, -math.inf)  # is_bst, size, min, max
        lb, ls, lmin, lmax = dfs(node.left)
        rb, rs, rmin, rmax = dfs(node.right)
        if lb and rb and lmax < node.val < rmin:
            size = ls + rs + 1
            best = max(best, size)
            return (True, size, min(lmin, node.val), max(rmax, node.val))
        return (False, 0, 0, 0)

    dfs(root)
    return best


if __name__ == "__main__":
    #        10
    #       /  \
    #      5    15
    #     / \     \
    #    1   8     7   -> largest BST is {1,5,8} size 3
    root = Node(10, Node(5, Node(1), Node(8)), Node(15, None, Node(7)))
    print(largest_bst(root))  # 3
