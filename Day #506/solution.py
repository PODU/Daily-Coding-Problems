# Day 506: Zigzag rearrange linked list values in a single pass by swapping adjacent
# node values that violate the expected ordering. Time O(n), Space O(1).


class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


def zigzag(head):
    less = True  # even index expects list[i] <= list[i+1]
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


def main():
    head = tail = None
    for v in [1, 2, 3, 4, 5]:
        n = Node(v)
        if head is None:
            head = tail = n
        else:
            tail.next = n
            tail = n
    zigzag(head)
    parts = []
    cur = head
    while cur:
        parts.append(str(cur.val))
        cur = cur.next
    print(" -> ".join(parts))


if __name__ == "__main__":
    main()
