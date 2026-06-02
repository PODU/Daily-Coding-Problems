# Day 1601: Min root-to-leaf path sum via recursive DFS; leaf returns its val, internal node adds min of existing children.
# Reconstruct path by tracking the chosen child. Time O(n), space O(h).


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def min_path(node):
    """Return (min_sum, path_list) from node down to a leaf."""
    if node.left is None and node.right is None:
        return node.val, [node.val]

    best_sum = None
    best_path = None
    for child in (node.left, node.right):
        if child is None:
            continue
        s, p = min_path(child)
        if best_sum is None or s < best_sum:
            best_sum, best_path = s, p
    return node.val + best_sum, [node.val] + best_path


def main():
    root = Node(10)
    root.left = Node(5)
    root.right = Node(5)
    root.left.right = Node(2)
    root.right.right = Node(1)
    root.right.right.left = Node(-1)

    total, path = min_path(root)
    print("The minimum path is [" + ", ".join(map(str, path)) +
          "], which has sum " + str(total) + ".")


if __name__ == "__main__":
    main()
