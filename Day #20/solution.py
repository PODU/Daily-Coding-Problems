# Day 20: Intersection of two linked lists: two-pointer switch trick.
# Time O(M+N), Space O(1).
class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


def get_intersection(a, b):
    pa, pb = a, b
    while pa is not pb:
        pa = b if pa is None else pa.next
        pb = a if pb is None else pb.next
    return pa


if __name__ == "__main__":
    shared = Node(8)
    shared.next = Node(10)

    a = Node(3)
    a.next = Node(7)
    a.next.next = shared

    b = Node(99)
    b.next = Node(1)
    b.next.next = shared

    res = get_intersection(a, b)
    print(res.val)
