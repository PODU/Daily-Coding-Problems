# Day 465: Reverse a singly linked list in-place by re-pointing each next pointer.
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


def to_str(head):
    parts = []
    while head:
        parts.append(str(head.val))
        head = head.next
    return " ".join(parts)


def main():
    head = Node(1)
    head.next = Node(2)
    head.next.next = Node(3)
    head.next.next.next = Node(4)
    head.next.next.next.next = Node(5)
    print(to_str(head))
    head = reverse(head)
    print(to_str(head))


if __name__ == "__main__":
    main()
