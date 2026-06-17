# Day 1679: Size of largest BST subtree. Post-order returns (isBST, size, min, max)
# per subtree, tracking the global best. Time O(n), Space O(h).
import math


class TreeNode:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def largest_bst(root):
    best = 0

    def dfs(node):
        nonlocal best
        if not node:
            return (True, 0, math.inf, -math.inf)
        l_bst, l_sz, l_mn, l_mx = dfs(node.left)
        r_bst, r_sz, r_mn, r_mx = dfs(node.right)
        if l_bst and r_bst and l_mx < node.val < r_mn:
            sz = l_sz + r_sz + 1
            best = max(best, sz)
            return (True, sz, min(node.val, l_mn), max(node.val, r_mx))
        return (False, 0, -math.inf, math.inf)

    dfs(root)
    return best


if __name__ == "__main__":
    root = TreeNode(10,
                    TreeNode(5, TreeNode(1), TreeNode(8)),
                    TreeNode(15, None, TreeNode(7)))
    print(largest_bst(root))  # 3
