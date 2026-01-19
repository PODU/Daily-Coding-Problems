# Day 917: Partition linked list: stable split into <k and >=k lists, then concatenate.
# Time O(n), Space O(1).
class Node:
    def __init__(self, val):
        self.val = val
        self.next = None

def partition(head, k):
    less_dummy = Node(0)
    ge_dummy = Node(0)
    lt = less_dummy
    ge = ge_dummy
    cur = head
    while cur:
        if cur.val < k:
            lt.next = cur
            lt = cur
        else:
            ge.next = cur
            ge = cur
        cur = cur.next
    ge.next = None
    lt.next = ge_dummy.next
    return less_dummy.next

def main():
    vals = [5, 1, 8, 0, 3]
    head = None
    tail = None
    for v in vals:
        n = Node(v)
        if head is None:
            head = tail = n
        else:
            tail.next = n
            tail = n
    head = partition(head, 3)
    parts = []
    cur = head
    while cur:
        parts.append(str(cur.val))
        cur = cur.next
    print(" -> ".join(parts))

if __name__ == "__main__":
    main()
