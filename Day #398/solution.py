# Day 398: Remove k-th node from end in one pass via two pointers + dummy head. O(n) time, O(1) space.
class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


def remove_kth_from_end(head, k):
    dummy = Node(0, head)
    fast = slow = dummy
    for _ in range(k):  # advance fast k ahead
        fast = fast.next
    while fast.next:
        fast = fast.next
        slow = slow.next
    slow.next = slow.next.next
    return dummy.next


def print_list(head):
    parts = []
    c = head
    while c:
        parts.append(str(c.val))
        c = c.next
    print(" -> ".join(parts))


if __name__ == "__main__":
    head = Node(1, Node(2, Node(3, Node(4, Node(5)))))
    head = remove_kth_from_end(head, 2)
    print_list(head)
