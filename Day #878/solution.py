# Day 878: Largest BST subtree via post-order returning (isBST, size, min, max). Time O(n), Space O(h).
import math


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def largest_bst(root):
    best = 0

    def dfs(n):
        nonlocal best
        if n is None:
            return (True, 0, math.inf, -math.inf)  # isBST, size, min, max
        l_ok, l_sz, l_mn, l_mx = dfs(n.left)
        r_ok, r_sz, r_mn, r_mx = dfs(n.right)
        if l_ok and r_ok and l_mx < n.val < r_mn:
            sz = l_sz + r_sz + 1
            best = max(best, sz)
            return (True, sz, min(n.val, l_mn), max(n.val, r_mx))
        return (False, 0, 0, 0)

    dfs(root)
    return best


if __name__ == "__main__":
    root = Node(10, Node(5, Node(1), Node(8)), Node(15, None, Node(7)))
    print(largest_bst(root))
