# Day 73: Reverse singly linked list in place: iterate with prev/curr/next pointers. Time O(n), Space O(1).
class ListNode:
    def __init__(self, val):
        self.val = val
        self.next = None


def reverse_list(head):
    prev = None
    while head:
        nxt = head.next
        head.next = prev
        prev = head
        head = nxt
    return prev


def main():
    head = ListNode(1)
    head.next = ListNode(2)
    head.next.next = ListNode(3)
    head.next.next.next = ListNode(4)
    head.next.next.next.next = ListNode(5)

    head = reverse_list(head)

    vals = []
    p = head
    while p:
        vals.append(str(p.val))
        p = p.next
    print(" ".join(vals))


if __name__ == "__main__":
    main()
