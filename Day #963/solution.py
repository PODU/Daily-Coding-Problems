# Day 963: Find intersecting node of two singly linked lists.
# Approach: two pointers swap heads; meet at intersection. Time O(M+N), Space O(1).


class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


def get_intersection(a, b):
    if not a or not b:
        return None
    p, q = a, b
    while p is not q:
        p = b if p is None else p.next
        q = a if q is None else q.next
    return p


if __name__ == '__main__':
    n8 = Node(8)
    n8.next = Node(10)
    a = Node(3); a.next = Node(7); a.next.next = n8
    b = Node(99); b.next = Node(1); b.next.next = n8

    res = get_intersection(a, b)
    print("the node with value " + str(res.val))
