# Day 517: Intersection: two pointers switch lists after end; meet at the join. O(M+N) time, O(1) space.
class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


def get_intersection(a, b):
    if not a or not b:
        return None
    p, q = a, b
    while p is not q:
        p = b if p is None else p.next
        q = a if q is None else q.next
    return p


if __name__ == "__main__":
    shared = Node(8, Node(10))
    A = Node(3, Node(7, shared))
    B = Node(99, Node(1, shared))
    print("The node with value", get_intersection(A, B).val)
