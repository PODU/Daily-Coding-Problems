# Day 1207: Remove kth-from-last node in one pass, constant space.
# Two pointers k apart; when lead hits end, trail is just before target. Time O(n), Space O(1).


class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


def remove_kth_last(head, k):
    dummy = Node(0, head)
    lead = trail = dummy
    for _ in range(k):
        lead = lead.next
    while lead.next:
        lead = lead.next
        trail = trail.next
    trail.next = trail.next.next
    return dummy.next


if __name__ == "__main__":
    head = Node(1, Node(2, Node(3, Node(4, Node(5)))))
    head = remove_kth_last(head, 2)  # remove 4
    out = []
    while head:
        out.append(str(head.val))
        head = head.next
    print(" -> ".join(out))  # 1 -> 2 -> 3 -> 5
