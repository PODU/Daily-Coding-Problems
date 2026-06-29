# Day 1735: Iterative in-place reversal of a singly linked list using three pointers.
# Time: O(n), Space: O(1).
class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


def reverse(head):
    prev = None
    while head:
        nxt = head.next
        head.next = prev
        prev = head
        head = nxt
    return prev


if __name__ == "__main__":
    head = None
    for i in range(5, 0, -1):
        n = Node(i)
        n.next = head
        head = n
    head = reverse(head)
    out = []
    c = head
    while c:
        out.append(str(c.val))
        c = c.next
    print(" ".join(out))
