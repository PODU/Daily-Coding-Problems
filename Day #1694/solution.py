# Day 1694: Zigzag list: single pass, even index wants lst[i]<=lst[i+1], odd wants lst[i]>=lst[i+1]; swap if violated.
# O(n) time, O(1) extra space.


class ListNode:
    def __init__(self, val):
        self.val = val
        self.next = None


def zigzag(head):
    cur = head
    i = 0
    while cur and cur.next:
        if i % 2 == 0:
            if cur.val > cur.next.val:
                cur.val, cur.next.val = cur.next.val, cur.val
        else:
            if cur.val < cur.next.val:
                cur.val, cur.next.val = cur.next.val, cur.val
        cur = cur.next
        i += 1


if __name__ == "__main__":
    head = ListNode(1)
    cur = head
    for v in [2, 3, 4, 5]:
        cur.next = ListNode(v)
        cur = cur.next

    zigzag(head)

    parts = []
    cur = head
    while cur:
        parts.append(str(cur.val))
        cur = cur.next
    print(" -> ".join(parts))
