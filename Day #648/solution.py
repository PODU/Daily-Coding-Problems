# Day 648: Deep clone linked list with random pointers using O(1) interleaving (3 passes).
# Time O(n), Space O(1) extra. Python3.

class Node:
    def __init__(self, val):
        self.val = val
        self.next = None
        self.random = None


def clone_list(head):
    if not head:
        return None
    # Pass 1: insert cloned node after each original
    cur = head
    while cur:
        copy = Node(cur.val)
        copy.next = cur.next
        cur.next = copy
        cur = copy.next
    # Pass 2: set clone.random
    cur = head
    while cur:
        cur.next.random = cur.random.next if cur.random else None
        cur = cur.next.next
    # Pass 3: split lists
    new_head = head.next
    cur = head
    while cur:
        copy = cur.next
        cur.next = copy.next
        copy.next = copy.next.next if copy.next else None
        cur = cur.next
    return new_head


def main():
    n1, n2, n3, n4 = Node(1), Node(2), Node(3), Node(4)
    n1.next = n2
    n2.next = n3
    n3.next = n4
    n1.random = n3
    n2.random = n1
    n3.random = n3
    n4.random = n2

    cloned = clone_list(n1)

    orig_ids = set()
    cur = n1
    while cur:
        orig_ids.add(id(cur))
        cur = cur.next

    deep = True
    cur = cloned
    while cur:
        print("node {} random {}".format(cur.val, cur.random.val if cur.random else 0))
        if id(cur) in orig_ids:
            deep = False
        if cur.random and id(cur.random) in orig_ids:
            deep = False
        cur = cur.next
    print("deep copy verified: {}".format("true" if deep else "false"))


if __name__ == "__main__":
    main()
