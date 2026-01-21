# Day 936: Sum of BST values within inclusive range [a,b], pruning branches out of range.
# Time O(n) worst, O(h + k) typical, Space O(h).


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def range_sum(n, a, b):
    if n is None:
        return 0
    if n.val < a:
        return range_sum(n.right, a, b)
    if n.val > b:
        return range_sum(n.left, a, b)
    return n.val + range_sum(n.left, a, b) + range_sum(n.right, a, b)


if __name__ == "__main__":
    root = Node(5,
                Node(3, Node(2), Node(4)),
                Node(8, Node(6), Node(10)))
    print(range_sum(root, 4, 9))  # 23
