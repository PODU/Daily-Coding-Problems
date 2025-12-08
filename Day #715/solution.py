# Day 715: Linked-list palindrome. Find middle (slow/fast), reverse second half,
# compare. Works for singly linked in O(n) time, O(1) space.

class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


def is_palindrome(head):
    if not head or not head.next:
        return True
    slow = fast = head
    while fast.next and fast.next.next:
        slow = slow.next
        fast = fast.next.next
    prev, cur = None, slow.next
    while cur:
        nx = cur.next
        cur.next = prev
        prev = cur
        cur = nx
    p1, p2 = head, prev
    while p2:
        if p1.val != p2.val:
            return False
        p1, p2 = p1.next, p2.next
    return True


def build(vals):
    head = None
    for x in reversed(vals):
        head = Node(x, head)
    return head


if __name__ == "__main__":
    print("True" if is_palindrome(build([1, 4, 3, 4, 1])) else "False")
    print("True" if is_palindrome(build([1, 4])) else "False")
