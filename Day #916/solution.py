# Day 916: Reverse a singly linked list in-place by re-pointing each node's next pointer.
# Time: O(n), Space: O(1).
class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


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
    return " -> ".join(parts)


if __name__ == "__main__":
    head = Node(1, Node(2, Node(3, Node(4, Node(5)))))
    head = reverse(head)
    print(to_str(head))  # 5 -> 4 -> 3 -> 2 -> 1
