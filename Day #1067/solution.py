# Day 1067: Post-order prune: remove subtrees consisting entirely of zeros. Time: O(n), Space: O(n) stack.

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val, self.left, self.right = val, left, right

def prune(node):
    if node is None:
        return None
    node.left  = prune(node.left)
    node.right = prune(node.right)
    if node.val == 0 and node.left is None and node.right is None:
        return None
    return node

def preorder(node, out):
    if node is None:
        out.append('X')
        return
    out.append(str(node.val))
    preorder(node.left,  out)
    preorder(node.right, out)

# Build tree
root = TreeNode(0,
    TreeNode(1),
    TreeNode(0,
        TreeNode(1, TreeNode(0), TreeNode(0)),
        TreeNode(0)
    )
)

root = prune(root)
out = []
preorder(root, out)
print("Pruned preorder (X=null):", ' '.join(out))
