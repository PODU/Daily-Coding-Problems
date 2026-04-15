# Day 1365: Count complete-tree nodes: if left height == right height subtree is perfect (2^h-1),
# else recurse. Time O(log^2 n), Space O(log n) recursion.

class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def left_height(n):
    h = 0
    while n:
        h += 1
        n = n.left
    return h


def right_height(n):
    h = 0
    while n:
        h += 1
        n = n.right
    return h


def count_nodes(root):
    if not root:
        return 0
    lh = left_height(root)
    rh = right_height(root)
    if lh == rh:
        return (1 << lh) - 1
    return 1 + count_nodes(root.left) + count_nodes(root.right)


def main():
    root = Node(1)
    root.left = Node(2)
    root.right = Node(3)
    root.left.left = Node(4)
    root.left.right = Node(5)
    root.right.left = Node(6)
    print(count_nodes(root))


if __name__ == "__main__":
    main()
