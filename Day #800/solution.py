# Day 800: Rearrange list values into low->high->low... (wiggle).
# One pass: at even idx ensure a<=next, at odd idx ensure a>=next; swap if not.
# Time O(N), Space O(1).


class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


def wiggle(head):
    less = True  # even index expects cur <= next
    cur = head
    while cur and cur.next:
        if (cur.val > cur.next.val) if less else (cur.val < cur.next.val):
            cur.val, cur.next.val = cur.next.val, cur.val
        less = not less
        cur = cur.next


if __name__ == "__main__":
    head = Node(1)
    c = head
    for v in [2, 3, 4, 5]:
        c.next = Node(v)
        c = c.next
    wiggle(head)
    out = []
    p = head
    while p:
        out.append(str(p.val))
        p = p.next
    print(" -> ".join(out))  # 1 -> 3 -> 2 -> 5 -> 4
