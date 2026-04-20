# Day 1394: Reverse singly linked list in-place: iterative 3-pointer (prev/cur/next). O(n) time, O(1) space.

class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


def reverse_list(head):
    prev, cur = None, head
    while cur:
        nxt = cur.next
        cur.next = prev
        prev = cur
        cur = nxt
    return prev


def main():
    head = Node(1)
    head.next = Node(2)
    head.next.next = Node(3)
    head.next.next.next = Node(4)
    head.next.next.next.next = Node(5)

    head = reverse_list(head)

    parts = []
    p = head
    while p:
        parts.append(str(p.val))
        p = p.next
    print(" -> ".join(parts))


if __name__ == "__main__":
    main()
