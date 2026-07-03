# Day 1755: Count nodes in a COMPLETE binary tree in better than O(n).
# Compare left/right spine heights: if equal, subtree is perfect (2^h - 1);
# else 1 + recurse on both children. Time O(log^2 n), Space O(log n).


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
        return (1 << lh) - 1  # perfect subtree
    return 1 + count_nodes(root.left) + count_nodes(root.right)


def main():
    # Complete binary tree with 6 nodes (values 1..6):
    #          1
    #        /   \
    #       2     3
    #      / \   /
    #     4   5 6
    root = Node(1)
    root.left = Node(2)
    root.right = Node(3)
    root.left.left = Node(4)
    root.left.right = Node(5)
    root.right.left = Node(6)

    print(count_nodes(root))  # expected: 6


if __name__ == "__main__":
    main()
