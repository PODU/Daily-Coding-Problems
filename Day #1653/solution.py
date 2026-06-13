# Day 1653: Evaluate arithmetic expression tree: recurse, combining children by operator
# at each internal node; leaves are integers. Time O(n), Space O(h) recursion.

class Node:
    def __init__(self, val=None, op=None, left=None, right=None):
        self.val = val
        self.op = op
        self.left = left
        self.right = right

    @property
    def is_leaf(self):
        return self.op is None


def evaluate(node):
    if node.is_leaf:
        return node.val
    a, b = evaluate(node.left), evaluate(node.right)
    if node.op == "+":
        return a + b
    if node.op == "-":
        return a - b
    if node.op == "*":
        return a * b
    if node.op == "/":
        return a // b


if __name__ == "__main__":
    left = Node(op="+", left=Node(3), right=Node(2))
    right = Node(op="+", left=Node(4), right=Node(5))
    root = Node(op="*", left=left, right=right)
    print(evaluate(root))
