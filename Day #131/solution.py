# Day 131: Deep clone a linked list with next + random pointers.
# Interleaving trick (weave copies, set randoms, unweave). O(n) time, O(1) extra space.


class Node:
    def __init__(self, val):
        self.val = val
        self.next = None
        self.random = None


def clone(head):
    if not head:
        return None
    c = head
    while c:
        cp = Node(c.val)
        cp.next = c.next
        c.next = cp
        c = cp.next
    c = head
    while c:
        if c.random:
            c.next.random = c.random.next
        c = c.next.next
    new_head = head.next
    c = head
    while c:
        cp = c.next
        c.next = cp.next
        if cp.next:
            cp.next = cp.next.next
        c = c.next
    return new_head


if __name__ == "__main__":
    n = [Node(v) for v in range(1, 6)]
    for i in range(4):
        n[i].next = n[i + 1]
    n[0].random = n[2]
    n[1].random = n[0]
    n[2].random = n[4]
    n[3].random = n[1]
    n[4].random = n[4]

    copy = clone(n[0])
    separate = True
    o, c = n[0], copy
    while c:
        if c is o:
            separate = False
        print("node %d -> random %d" % (c.val, c.random.val if c.random else 0))
        o = o.next
        c = c.next
    print("deep copy verified: " + ("true" if separate else "false"))
