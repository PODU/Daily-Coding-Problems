# Day 1638: Swap every two adjacent nodes in a singly linked list via iterative pointer swaps.
# Time: O(n), Space: O(1).

class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


def swap_pairs(head):
    dummy = Node(0)
    dummy.next = head
    prev = dummy
    while prev.next and prev.next.next:
        a = prev.next
        b = a.next
        a.next = b.next
        b.next = a
        prev.next = b
        prev = a
    return dummy.next


def main():
    head = Node(1)
    head.next = Node(2)
    head.next.next = Node(3)
    head.next.next.next = Node(4)

    head = swap_pairs(head)

    parts = []
    cur = head
    while cur:
        parts.append(str(cur.val))
        cur = cur.next
    print(" -> ".join(parts))


if __name__ == "__main__":
    main()
