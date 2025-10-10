# Day 405: Largest BST subtree: bottom-up DFS returning (isBST, size, min, max); combine children.
# Time O(n), Space O(h).
import math


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def largest_bst(root):
    best = 0

    def dfs(node):
        nonlocal best
        if node is None:
            return True, 0, math.inf, -math.inf
        l_bst, l_size, l_min, l_max = dfs(node.left)
        r_bst, r_size, r_min, r_max = dfs(node.right)
        if l_bst and r_bst and l_max < node.val < r_min:
            size = l_size + r_size + 1
            best = max(best, size)
            return True, size, min(node.val, l_min), max(node.val, r_max)
        return False, 0, 0, 0

    dfs(root)
    return best


if __name__ == "__main__":
    root = Node(10,
                Node(5, Node(1), Node(8)),
                Node(15, None, Node(7)))
    print(largest_bst(root))
