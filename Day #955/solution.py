# Day 955: evaluate an arithmetic expression binary tree (leaves=ints, nodes=+ - * /).
# Recursive post-order evaluation. Time O(n), Space O(h).

class Node:
    def __init__(self, val=None, op=None, left=None, right=None):
        self.val = val
        self.op = op
        self.left = left
        self.right = right


def evaluate(n):
    if n.left is None and n.right is None:
        return n.val
    a, b = evaluate(n.left), evaluate(n.right)
    if n.op == '+':
        return a + b
    if n.op == '-':
        return a - b
    if n.op == '*':
        return a * b
    return a / b


if __name__ == "__main__":
    root = Node(op='*',
                left=Node(op='+', left=Node(3), right=Node(2)),
                right=Node(op='+', left=Node(4), right=Node(5)))
    print(int(evaluate(root)))  # 45
