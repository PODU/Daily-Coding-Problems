# Day 765: Remove the kth-from-last node in one pass with two pointers.
# fast leads slow by k; when fast hits the end, slow precedes the target.
# Time: O(n), Space: O(1).


class Node:
    def __init__(self, val, nxt=None):
        self.val = val
        self.next = nxt


def remove_kth_last(head, k):
    dummy = Node(0, head)
    fast = slow = dummy
    for _ in range(k):
        fast = fast.next
    while fast.next:
        fast = fast.next
        slow = slow.next
    slow.next = slow.next.next   # unlink target
    return dummy.next


def print_list(head):
    parts = []
    p = head
    while p:
        parts.append(str(p.val))
        p = p.next
    print(" -> ".join(parts))


if __name__ == "__main__":
    head = Node(1)
    cur = head
    for v in range(2, 6):
        cur.next = Node(v)
        cur = cur.next

    print("before:", end=" ")
    print_list(head)
    head = remove_kth_last(head, 2)
    print("after: ", end=" ")
    print_list(head)   # 1 -> 2 -> 3 -> 5
