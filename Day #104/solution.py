# Day 104: Linked-list palindrome. Works for singly OR doubly linked: find middle
# (slow/fast), reverse second half, compare. O(n) time, O(1) extra space.
class Node:
    def __init__(self, val, nxt=None):
        self.val, self.next = val, nxt


def is_palindrome(head):
    # find middle
    slow = fast = head
    while fast and fast.next:
        slow, fast = slow.next, fast.next.next
    # reverse second half
    prev = None
    while slow:
        slow.next, prev, slow = prev, slow, slow.next
    # compare halves
    left, right = head, prev
    while right:
        if left.val != right.val:
            return False
        left, right = left.next, right.next
    return True


def build(vals):
    head = None
    for v in reversed(vals):
        head = Node(v, head)
    return head


if __name__ == "__main__":
    print(is_palindrome(build([1, 4, 3, 4, 1])))  # True
    print(is_palindrome(build([1, 4])))           # False
