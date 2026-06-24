# Day 1710: Deep clone list w/ random ptr: interleave clones, wire randoms, unweave. O(n) time, O(1) extra.
class Node:
    def __init__(self, val):
        self.val = val
        self.next = None
        self.random = None


def copy_random_list(head):
    if not head:
        return None
    c = head
    while c:
        cl = Node(c.val)
        cl.next = c.next
        c.next = cl
        c = cl.next
    c = head
    while c:
        c.next.random = c.random.next if c.random else None
        c = c.next.next
    new_head = head.next
    c = head
    while c:
        cl = c.next
        c.next = cl.next
        cl.next = cl.next.next if cl.next else None
        c = c.next
    return new_head


if __name__ == "__main__":
    n1, n2, n3, n4 = Node(1), Node(2), Node(3), Node(4)
    n1.next, n2.next, n3.next = n2, n3, n4
    n1.random = n3
    n2.random = n1
    n3.random = n3
    n4.random = n2

    cloned = copy_random_list(n1)
    c = cloned
    while c:
        print(f"node {c.val}, random {c.random.val}")
        c = c.next
