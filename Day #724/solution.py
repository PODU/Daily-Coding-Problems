# Day 724: Evaluate an arithmetic expression binary tree.
# Approach: Post-order recursion; leaves are ints, internal nodes are operators.
# Time: O(n), Space: O(h).

class Node:
    def __init__(self, val=None, op=None, left=None, right=None):
        self.val = val
        self.op = op
        self.left = left
        self.right = right


def evaluate(root):
    if root.left is None and root.right is None:
        return root.val
    l, r = evaluate(root.left), evaluate(root.right)
    return {
        '+': l + r,
        '-': l - r,
        '*': l * r,
        '/': l / r,
    }[root.op]


if __name__ == "__main__":
    tree = Node(op='*',
                left=Node(op='+', left=Node(3), right=Node(2)),
                right=Node(op='+', left=Node(4), right=Node(5)))
    print(int(evaluate(tree)))  # 45
