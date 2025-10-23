# Day 482: BST range sum [a,b] inclusive via DFS with pruning.
# Skip left subtree if node<a, skip right if node>b. Time O(n) worst, O(k+h) typical; Space O(h).
class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def range_sum(root, a, b):
    if not root:
        return 0
    if root.val < a:
        return range_sum(root.right, a, b)
    if root.val > b:
        return range_sum(root.left, a, b)
    return root.val + range_sum(root.left, a, b) + range_sum(root.right, a, b)


if __name__ == "__main__":
    root = Node(5,
                Node(3, Node(2), Node(4)),
                Node(8, Node(6), Node(10)))
    print(range_sum(root, 4, 9))  # 23
