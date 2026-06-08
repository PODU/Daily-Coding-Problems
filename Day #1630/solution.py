# Day 1630: Root-to-leaf paths via DFS, carrying current path; record at leaves. Time O(n), Space O(h).
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
    print(root_to_leaf_paths(root))
