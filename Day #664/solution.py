# Day 664: Maximum path sum between any two nodes in a binary tree.
# Post-order DFS: each node returns best downward gain; track best "bridge". Time O(n), Space O(h).


class Node:
    def __init__(self, val, l=None, r=None):
        self.val, self.l, self.r = val, l, r


def max_path_sum(root):
    best = float("-inf")

    def gain(n):
        nonlocal best
        if not n:
            return 0
        lg = max(0, gain(n.l))
        rg = max(0, gain(n.r))
        best = max(best, n.val + lg + rg)
        return n.val + max(lg, rg)

    gain(root)
    return best


if __name__ == "__main__":
    root = Node(-10, Node(9), Node(20, Node(15), Node(7)))
    print(max_path_sum(root))  # 42
