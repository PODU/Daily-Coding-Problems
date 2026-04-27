# Day 1432: Deep clone a linked list with a random pointer.
# Approach: interleave cloned nodes, wire randoms, then split. Time: O(n), Space: O(1) extra.


class Node:
    def __init__(self, val):
        self.val = val
        self.next = None
        self.random = None


def clone_list(head):
    if not head:
        return None
    # 1. interleave
    cur = head
    while cur:
        copy = Node(cur.val)
        copy.next = cur.next
        cur.next = copy
        cur = copy.next
    # 2. set randoms
    cur = head
    while cur:
        if cur.random:
            cur.next.random = cur.random.next
        cur = cur.next.next
    # 3. split
    new_head = head.next
    cur = head
    while cur:
        copy = cur.next
        cur.next = copy.next
        if copy.next:
            copy.next = copy.next.next
        cur = cur.next
    return new_head


if __name__ == "__main__":
    a, b, c = Node(1), Node(2), Node(3)
    a.next, b.next = b, c
    a.random, b.random, c.random = c, a, c

    cloned = clone_list(a)
    ok = True
    p, q = a, cloned
    while p:
        if q is p:
            ok = False
        if q.val != p.val:
            ok = False
        if (p.random is None) != (q.random is None):
            ok = False
        if p.random and q.random.val != p.random.val:
            ok = False
        p, q = p.next, q.next
    print("Clone verified: values and random targets match, nodes distinct"
          if ok else "Clone FAILED")
