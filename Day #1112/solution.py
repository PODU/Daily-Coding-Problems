# Day 1112: Day 1112 - Minimum root-to-leaf path sum (return the path)
# Approach: DFS, track best leaf path by sum. Time: O(n), Space: O(h).

class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def min_path(root):
    best = {"sum": float("inf"), "path": []}

    def dfs(node, path, s):
        if node is None:
            return
        path.append(node.val)
        s += node.val
        if node.left is None and node.right is None:
            if s < best["sum"]:
                best["sum"] = s
                best["path"] = list(path)
        else:
            dfs(node.left, path, s)
            dfs(node.right, path, s)
        path.pop()

    dfs(root, [], 0)
    return best["path"], best["sum"]


if __name__ == "__main__":
    root = Node(10,
                Node(5, None, Node(2)),
                Node(5, None, Node(1, Node(-1))))
    path, total = min_path(root)
    print(f"{path}, which has sum {total}")  # [10, 5, 1, -1], which has sum 15
