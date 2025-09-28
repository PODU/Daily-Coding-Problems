# Day 343: Range Sum of BST. Pruned DFS using BST property. Time O(n) worst, Space O(h).
class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def range_sum(root, a, b):
    if not root:
        return 0
    s = 0
    if a <= root.val <= b:
        s += root.val
    if root.val > a:
        s += range_sum(root.left, a, b)
    if root.val < b:
        s += range_sum(root.right, a, b)
    return s


def main():
    root = Node(5)
    root.left = Node(3)
    root.left.left = Node(2)
    root.left.right = Node(4)
    root.right = Node(8)
    root.right.left = Node(6)
    root.right.right = Node(10)
    print(range_sum(root, 4, 9))


if __name__ == "__main__":
    main()
