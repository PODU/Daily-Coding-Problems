# Day 904: DFS tracking depth; record the node value seen at maximum depth. Time O(n), Space O(h).
class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def deepest_node(root):
    best = {"depth": -1, "val": None}

    def dfs(node, depth):
        if not node:
            return
        if depth > best["depth"]:
            best["depth"] = depth
            best["val"] = node.val
        dfs(node.left, depth + 1)
        dfs(node.right, depth + 1)

    dfs(root, 0)
    return best["val"]


if __name__ == "__main__":
    a = Node("a", Node("b", Node("d")), Node("c"))
    print(deepest_node(a))
