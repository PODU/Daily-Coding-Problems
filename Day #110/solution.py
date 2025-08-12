# Day 110: Root-to-leaf paths via DFS backtracking. O(n) nodes, O(h) stack.
class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def root_to_leaf(root):
    res = []

    def dfs(n, path):
        if not n:
            return
        path.append(n.val)
        if not n.left and not n.right:
            res.append(list(path))
        else:
            dfs(n.left, path)
            dfs(n.right, path)
        path.pop()

    dfs(root, [])
    return res


if __name__ == "__main__":
    root = Node(1, Node(2), Node(3, Node(4), Node(5)))
    print(root_to_leaf(root))
