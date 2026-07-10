# Day 1792: Binary tree max path sum: DFS returning max downward gain; track global max = node + max(0,left) + max(0,right).
# Time O(n), Space O(h).
class Node:
    def __init__(self, val, left=None, right=None):
        self.val, self.left, self.right = val, left, right

def max_path_sum(root):
    best = float("-inf")
    def gain(n):
        nonlocal best
        if not n:
            return 0
        l = max(0, gain(n.left))
        r = max(0, gain(n.right))
        best = max(best, n.val + l + r)
        return n.val + max(l, r)
    gain(root)
    return best

if __name__ == "__main__":
    root = Node(-10, Node(9), Node(20, Node(15), Node(7)))
    print(max_path_sum(root))  # expected 42
