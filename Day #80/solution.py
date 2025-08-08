# Day 80: Return a deepest node of a binary tree via DFS tracking depth.
# Time O(n), Space O(h).


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def deepest_node(root):
    best = [-1, None]

    def dfs(node, depth):
        if not node:
            return
        if depth > best[0]:
            best[0], best[1] = depth, node.val
        dfs(node.left, depth + 1)
        dfs(node.right, depth + 1)

    dfs(root, 0)
    return best[1]


if __name__ == "__main__":
    a = Node("a", Node("b", Node("d")), Node("c"))
    print(deepest_node(a))  # d
