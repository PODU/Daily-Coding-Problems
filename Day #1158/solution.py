# Day 1158: Bottom-up iterative merge sort on a singly linked list. O(n log n) time, O(1) auxiliary space.


class ListNode:
    def __init__(self, val):
        self.val = val
        self.next = None


def split(head, n):
    # Take n nodes from head, cut, return remainder.
    i = 1
    while head and i < n:
        head = head.next
        i += 1
    if head is None:
        return None
    second = head.next
    head.next = None
    return second


def merge(a, b, tail):
    cur = tail
    while a and b:
        if a.val <= b.val:
            cur.next = a
            a = a.next
        else:
            cur.next = b
            b = b.next
        cur = cur.next
    cur.next = a if a else b
    while cur.next:
        cur = cur.next
    return cur


def sort_list(head):
    if head is None or head.next is None:
        return head
    n = 0
    p = head
    while p:
        n += 1
        p = p.next

    dummy = ListNode(0)
    dummy.next = head
    size = 1
    while size < n:
        cur = dummy.next
        tail = dummy
        while cur:
            left = cur
            right = split(left, size)
            cur = split(right, size)
            tail = merge(left, right, tail)
        size <<= 1
    return dummy.next


def main():
    vals = [4, 1, -3, 99]
    head = None
    tail = None
    for v in vals:
        node = ListNode(v)
        if head is None:
            head = tail = node
        else:
            tail.next = node
            tail = node

    head = sort_list(head)

    parts = []
    p = head
    while p:
        parts.append(str(p.val))
        p = p.next
    print(" -> ".join(parts))


if __name__ == "__main__":
    main()
