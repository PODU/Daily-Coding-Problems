# Day 1154: Minimum root-to-leaf path sum.
# DFS tracking running sum/path, keep best at leaves. O(n) time, O(h) space.
class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def min_path(root):
    best = {"sum": float("inf"), "path": []}

    def dfs(n, path, total):
        if not n:
            return
        path.append(n.val)
        total += n.val
        if not n.left and not n.right:
            if total < best["sum"]:
                best["sum"] = total
                best["path"] = path[:]
        else:
            dfs(n.left, path, total)
            dfs(n.right, path, total)
        path.pop()

    dfs(root, [], 0)
    return best["path"], best["sum"]


if __name__ == "__main__":
    root = Node(10, Node(5, right=Node(2)), Node(5, right=Node(1, left=Node(-1))))
    path, total = min_path(root)
    print(f"{path}, which has sum {total}")  # [10, 5, 1, -1], which has sum 15
