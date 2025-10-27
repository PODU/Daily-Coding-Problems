# Day 503: Sort a singly linked list using bottom-up (iterative) merge sort.
# Time O(n log n), Space O(1) auxiliary (no recursion).


class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


def list_length(head):
    n = 0
    while head:
        n += 1
        head = head.next
    return n


def split(head, size):
    """Split off `size` nodes from head; cut there and return the rest."""
    i = 1
    while head and i < size:
        head = head.next
        i += 1
    if not head:
        return None
    rest = head.next
    head.next = None
    return rest


def merge_lists(a, b, tail):
    """Merge two sorted lists after `tail`; return new tail."""
    while a and b:
        if a.val <= b.val:
            tail.next = a
            a = a.next
        else:
            tail.next = b
            b = b.next
        tail = tail.next
    tail.next = a if a else b
    while tail.next:
        tail = tail.next
    return tail


def sort_list(head):
    if not head or not head.next:
        return head
    n = list_length(head)
    dummy = Node(0)
    dummy.next = head
    size = 1
    while size < n:
        tail = dummy
        cur = dummy.next
        while cur:
            left = cur
            right = split(left, size)
            cur = split(right, size)
            tail = merge_lists(left, right, tail)
        size *= 2
    return dummy.next


def print_list(head):
    parts = []
    while head:
        parts.append(str(head.val))
        head = head.next
    print(" -> ".join(parts))


def main():
    vals = [4, 1, -3, 99]
    dummy = Node(0)
    tail = dummy
    for v in vals:
        tail.next = Node(v)
        tail = tail.next
    sorted_head = sort_list(dummy.next)
    print_list(sorted_head)  # -3 -> 1 -> 4 -> 99


if __name__ == "__main__":
    main()
