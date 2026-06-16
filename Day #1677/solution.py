# Day 1677: Linked-list palindrome. Singly: find middle, reverse 2nd half, compare.
# Doubly: two pointers from both ends. Time O(n), Space O(1).


class Node:
    def __init__(self, val):
        self.val = val
        self.next = None
        self.prev = None


def build(values):
    head = tail = None
    for x in values:
        n = Node(x)
        if head is None:
            head = tail = n
        else:
            tail.next = n
            n.prev = tail
            tail = n
    return head


def is_palindrome(head):
    slow = fast = head
    while fast and fast.next:
        slow = slow.next
        fast = fast.next.next
    prev = None
    while slow:
        nx = slow.next
        slow.next = prev
        prev = slow
        slow = nx
    a, b = head, prev
    while b:
        if a.val != b.val:
            return False
        a, b = a.next, b.next
    return True


if __name__ == "__main__":
    print(is_palindrome(build([1, 4, 3, 4, 1])))  # True
    print(is_palindrome(build([1, 4])))           # False
