# Day 94: Max path sum in a binary tree. DFS returns best downward gain; at each
# node consider a path bending through it (left+node+right). O(n) time, O(h) space.
class Node:
    def __init__(self, val, left=None, right=None):
        self.val, self.left, self.right = val, left, right


def max_path_sum(root):
    best = float('-inf')

    def gain(node):
        nonlocal best
        if not node:
            return 0
        l = max(gain(node.left), 0)   # drop negative branches
        r = max(gain(node.right), 0)
        best = max(best, node.val + l + r)
        return node.val + max(l, r)

    gain(root)
    return best


if __name__ == "__main__":
    #     -10
    #     /  \
    #    9   20
    #        / \
    #       15  7   -> best path 15 + 20 + 7 = 42
    root = Node(-10, Node(9), Node(20, Node(15), Node(7)))
    print(max_path_sum(root))  # 42
