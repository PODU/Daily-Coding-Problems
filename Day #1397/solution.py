# Day 1397: Two-pointer: advance pa/pb; on reaching end switch to other head.
# They meet at intersection after at most M+N steps. Time O(M+N), Space O(1).

class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


def get_intersection(a, b):
    if not a or not b:
        return None
    pa, pb = a, b
    while pa is not pb:
        pa = b if pa is None else pa.next
        pb = a if pb is None else pb.next
    return pa


if __name__ == "__main__":
    shared = Node(8)
    shared.next = Node(10)
    a = Node(3); a.next = Node(7); a.next.next = shared
    b = Node(99); b.next = Node(1); b.next.next = shared
    print("the node with value", get_intersection(a, b).val)
