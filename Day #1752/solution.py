# Day 1752: Remove kth-from-last node of a singly linked list in ONE pass, O(1) space.
# Two pointers spaced k apart; when fast reaches end, slow is just before the target. O(n) time.
# Assumption (no README example): list 1->2->3->4->5, k=2 removes value 4 -> "1 2 3 5".


class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


def remove_kth_last(head, k):
    dummy = Node(0)
    dummy.next = head
    fast = slow = dummy
    for _ in range(k):  # advance fast k steps
        fast = fast.next
    while fast.next:
        fast = fast.next
        slow = slow.next
    slow.next = slow.next.next  # unlink target
    return dummy.next


def build(values):
    dummy = Node(0)
    cur = dummy
    for v in values:
        cur.next = Node(v)
        cur = cur.next
    return dummy.next


if __name__ == "__main__":
    head = build([1, 2, 3, 4, 5])
    head = remove_kth_last(head, 2)
    out = []
    while head:
        out.append(str(head.val))
        head = head.next
    print(" ".join(out))  # 1 2 3 5
