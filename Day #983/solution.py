# Day 983: Root-to-leaf paths via DFS, appending to the current path and recording it at each leaf.
# Time O(n) nodes (O(n*h) including path copies), Space O(h) recursion.


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def root_to_leaf_paths(root):
    res = []

    def dfs(node, path):
        if not node:
            return
        path.append(node.val)
        if not node.left and not node.right:
            res.append(list(path))
        else:
            dfs(node.left, path)
            dfs(node.right, path)
        path.pop()

    dfs(root, [])
    return res


if __name__ == "__main__":
    root = Node(1, Node(2), Node(3, Node(4), Node(5)))
    print(root_to_leaf_paths(root))  # [[1, 2], [1, 3, 4], [1, 3, 5]]
