# Day 1425: evaluate an arithmetic expression binary tree (+,-,*,/ internal; ints at leaves).
# Approach: post-order recursion, combine children by operator. O(n) time, O(h) space.


class Node:
    def __init__(self, op=None, val=0, left=None, right=None):
        self.op = op
        self.val = val
        self.left = left
        self.right = right


def leaf(v):
    return Node(val=v)


def eval_tree(n):
    if n.left is None and n.right is None:
        return n.val
    a, b = eval_tree(n.left), eval_tree(n.right)
    if n.op == "+":
        return a + b
    if n.op == "-":
        return a - b
    if n.op == "*":
        return a * b
    return a / b


if __name__ == "__main__":
    root = Node("*", left=Node("+", left=leaf(3), right=leaf(2)),
                right=Node("+", left=leaf(4), right=leaf(5)))
    print(int(eval_tree(root)))  # 45
