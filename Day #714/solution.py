# Day 714: Swap every two adjacent nodes in a singly linked list. Iterative
# pointer rewiring with a dummy head. Time O(n), space O(1).

class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


def swap_pairs(head):
    dummy = Node(0, head)
    prev = dummy
    while prev.next and prev.next.next:
        a = prev.next
        b = a.next
        a.next = b.next
        b.next = a
        prev.next = b
        prev = a
    return dummy.next


if __name__ == "__main__":
    head = Node(1, Node(2, Node(3, Node(4))))
    head = swap_pairs(head)
    out = []
    c = head
    while c:
        out.append(str(c.val))
        c = c.next
    print(" -> ".join(out))
