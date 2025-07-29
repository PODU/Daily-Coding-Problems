# Day 50: Evaluate arithmetic expression binary tree via post-order recursion.
# Time: O(n), Space: O(h).
import operator


class Node:
    def __init__(self, val=None, op=None, left=None, right=None):
        self.val = val
        self.op = op
        self.left = left
        self.right = right


OPS = {"+": operator.add, "-": operator.sub, "*": operator.mul, "/": operator.truediv}


def evaluate(n):
    if n.op is None:
        return n.val
    return OPS[n.op](evaluate(n.left), evaluate(n.right))


if __name__ == "__main__":
    root = Node(op="*",
                left=Node(op="+", left=Node(3), right=Node(2)),
                right=Node(op="+", left=Node(4), right=Node(5)))
    print(int(evaluate(root)))
