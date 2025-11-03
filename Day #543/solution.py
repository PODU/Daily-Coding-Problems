# Day 543: Remove kth-from-last node in one pass via two pointers k apart. O(n) time, O(1) space.
class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


def remove_kth_last(head, k):
    dummy = Node(0, head)
    fast = slow = dummy
    for _ in range(k):
        fast = fast.next
    while fast.next:
        fast = fast.next
        slow = slow.next
    slow.next = slow.next.next
    return dummy.next


if __name__ == "__main__":
    head = Node(1, Node(2, Node(3, Node(4, Node(5)))))
    head = remove_kth_last(head, 2)
    out = []
    p = head
    while p:
        out.append(str(p.val))
        p = p.next
    print(" ".join(out))
