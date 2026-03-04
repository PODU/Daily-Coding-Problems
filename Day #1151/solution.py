# Day 1151: Palindrome linked list (singly).
# Find middle via slow/fast, reverse 2nd half, compare. O(n) time, O(1) space.
class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


def is_palindrome(head):
    if not head or not head.next:
        return True
    slow = fast = head
    while fast.next and fast.next.next:
        slow = slow.next
        fast = fast.next.next
    prev, cur = None, slow.next
    while cur:
        prev, cur.next, cur = cur, prev, cur.next
    p1, p2 = head, prev
    while p2:
        if p1.val != p2.val:
            return False
        p1, p2 = p1.next, p2.next
    return True


def build(vals):
    dummy = t = Node(0)
    for x in vals:
        t.next = Node(x)
        t = t.next
    return dummy.next


if __name__ == "__main__":
    print(is_palindrome(build([1, 4, 3, 4, 1])))  # True
    print(is_palindrome(build([1, 4])))           # False
