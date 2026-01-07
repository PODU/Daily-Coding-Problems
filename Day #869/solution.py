# Day 869: Is a linked list a palindrome (works for singly linked; doubly trivial).
# Approach: find middle (slow/fast), reverse second half, compare both halves. O(1) space.
# Time: O(n), Space: O(1).


class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


def build(vals):
    head = tail = None
    for v in vals:
        n = Node(v)
        if head is None:
            head = tail = n
        else:
            tail.next = n
            tail = n
    return head


def is_palindrome(head):
    slow = fast = head
    while fast and fast.next:
        slow = slow.next
        fast = fast.next.next
    prev = None
    while slow:
        slow.next, prev, slow = prev, slow, slow.next
    a, b = head, prev
    while b:
        if a.val != b.val:
            return False
        a, b = a.next, b.next
    return True


if __name__ == "__main__":
    print(is_palindrome(build([1, 4, 3, 4, 1])))  # True
    print(is_palindrome(build([1, 4])))           # False
