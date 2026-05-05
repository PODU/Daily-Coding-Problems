# Day 1478: Return a deepest node of a binary tree.
# Single DFS returning (depth, node); keep the deeper subtree's result.
# Time O(N), Space O(H).

class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def deepest(root):
    def dfs(node):
        if node is None:
            return (0, None)
        ld, ln = dfs(node.left)
        rd, rn = dfs(node.right)
        if ld >= rd:
            return (ld + 1, ln if node.left else node)
        return (rd + 1, rn)
    return dfs(root)[1]


if __name__ == "__main__":
    # a / (b, c), b / (d)
    root = Node('a', Node('b', Node('d')), Node('c'))
    print(deepest(root).val)  # d
