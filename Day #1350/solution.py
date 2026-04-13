# Day 1350: Remove consecutive nodes summing to zero: dummy head, prefix-sum -> last node map;
# repeated prefix means a zero-sum span to splice out. Time O(n), Space O(n).


class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


def remove_zero_sum(head):
    dummy = Node(0)
    dummy.next = head
    seen = {}
    prefix = 0
    cur = dummy
    while cur:
        prefix += cur.val
        seen[prefix] = cur  # last node achieving this prefix sum
        cur = cur.next
    prefix = 0
    cur = dummy
    while cur:
        prefix += cur.val
        cur.next = seen[prefix].next  # skip zero-sum span
        cur = cur.next
    return dummy.next


def build(vals):
    head = tail = None
    for v in vals:
        n = Node(v)
        if head is None:
            head = tail = n
        else:
            tail.next = n
            tail = n
    return head


if __name__ == "__main__":
    head = build([3, 4, -7, 5, -6, 6])
    head = remove_zero_sum(head)
    parts = []
    c = head
    while c:
        parts.append(str(c.val))
        c = c.next
    print(" -> ".join(parts))
