# Day 966: Deep clone a linked list where each node has a random pointer.
# Approach: interleave copies, set randoms, split. Time O(n), Space O(1) extra.


class Node:
    def __init__(self, val):
        self.val = val
        self.next = None
        self.random = None


def clone_list(head):
    if not head:
        return None
    p = head
    while p:
        c = Node(p.val)
        c.next = p.next
        p.next = c
        p = c.next
    p = head
    while p:
        if p.random:
            p.next.random = p.random.next
        p = p.next.next
    new_head = head.next
    p = head
    while p:
        c = p.next
        p.next = c.next
        if c.next:
            c.next = c.next.next
        p = p.next
    return new_head


if __name__ == '__main__':
    a, b, c = Node(1), Node(2), Node(3)
    a.next = b; b.next = c
    a.random = c; b.random = a; c.random = b

    cl = clone_list(a)
    p = cl
    while p:
        print("val=%d random=%d" % (p.val, p.random.val if p.random else -1))
        p = p.next
