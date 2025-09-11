# Day 256: Rearrange linked list values into zigzag low->high->low form.
# One pass over node values: at even i ensure a[i]<=a[i+1], at odd i ensure a[i]>=a[i+1], swap on violation.
# Time: O(n), Space: O(1).


class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


def wiggle(head):
    less = True  # even index: want current <= next
    cur = head
    while cur and cur.next:
        if less:
            if cur.val > cur.next.val:
                cur.val, cur.next.val = cur.next.val, cur.val
        else:
            if cur.val < cur.next.val:
                cur.val, cur.next.val = cur.next.val, cur.val
        less = not less
        cur = cur.next


def list_to_string(head):
    parts = []
    cur = head
    while cur:
        parts.append(str(cur.val))
        cur = cur.next
    return " -> ".join(parts)


def main():
    vals = [1, 2, 3, 4, 5]
    head = tail = None
    for v in vals:
        n = Node(v)
        if head is None:
            head = tail = n
        else:
            tail.next = n
            tail = n
    wiggle(head)
    print(list_to_string(head))  # 1 -> 3 -> 2 -> 5 -> 4


if __name__ == "__main__":
    main()
