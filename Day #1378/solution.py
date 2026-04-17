# Day 1378: Max path sum between any two nodes via DFS returning best downward gain.
# Time O(n), Space O(h).


class TreeNode:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def max_path_sum(root):
    best = float("-inf")

    def gain(n):
        nonlocal best
        if not n:
            return 0
        l = max(0, gain(n.left))
        r = max(0, gain(n.right))
        best = max(best, n.val + l + r)
        return n.val + max(l, r)

    gain(root)
    return best


if __name__ == "__main__":
    root = TreeNode(-10, TreeNode(9), TreeNode(20, TreeNode(15), TreeNode(7)))
    print(max_path_sum(root))  # 42
