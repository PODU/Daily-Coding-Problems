# Day 1078: Postorder DFS: each node returns val+max(0,L,R) upward; global best = val+max(0,L)+max(0,R); O(n) time O(h) space

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def maxPathSum(root):
    best = [float('-inf')]

    def dfs(node):
        if not node:
            return 0
        l = max(0, dfs(node.left))
        r = max(0, dfs(node.right))
        best[0] = max(best[0], node.val + l + r)
        return node.val + max(l, r)

    dfs(root)
    return best[0]

#       -10
#       /  \
#      9    20
#          /  \
#         15   7
root1 = TreeNode(-10)
root1.left = TreeNode(9)
root1.right = TreeNode(20)
root1.right.left = TreeNode(15)
root1.right.right = TreeNode(7)
print(f"Max path sum: {maxPathSum(root1)}")

#    1
#   / \
#  2   3
root2 = TreeNode(1)
root2.left = TreeNode(2)
root2.right = TreeNode(3)
print(f"Max path sum: {maxPathSum(root2)}")
