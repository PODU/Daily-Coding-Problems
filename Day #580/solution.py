# Day 580: Min root-to-leaf path sum via DFS, reconstructing the path. Time O(n), Space O(h).
class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def min_path(root):
    best = {"sum": float("inf"), "path": []}

    def dfs(node, cur):
        if node is None:
            return
        cur.append(node.val)
        if node.left is None and node.right is None:
            s = sum(cur)
            if s < best["sum"]:
                best["sum"] = s
                best["path"] = list(cur)
        else:
            dfs(node.left, cur)
            dfs(node.right, cur)
        cur.pop()

    dfs(root, [])
    return best["path"], best["sum"]


if __name__ == "__main__":
    root = Node(10)
    root.left = Node(5)
    root.left.right = Node(2)
    root.right = Node(5)
    root.right.right = Node(1)
    root.right.right.left = Node(-1)

    path, total = min_path(root)
    print("[" + ", ".join(str(x) for x in path) + "], which has sum " + str(total) + ".")
