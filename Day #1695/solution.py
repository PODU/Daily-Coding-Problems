# Day 1695: Two-pointer intersection of singly linked lists: redirect each pointer to the other head at end.
# Time O(M+N), Space O(1).

class Node:
    def __init__(self, val):
        self.val = val
        self.next = None


def get_intersection(head_a, head_b):
    if not head_a or not head_b:
        return None
    p_a, p_b = head_a, head_b
    while p_a is not p_b:
        p_a = head_b if p_a is None else p_a.next
        p_b = head_a if p_b is None else p_b.next
    return p_a


def main():
    n8 = Node(8)
    n8.next = Node(10)
    a = Node(3)
    a.next = Node(7)
    a.next.next = n8
    b = Node(99)
    b.next = Node(1)
    b.next.next = n8

    res = get_intersection(a, b)
    print(res.val if res else "null")


if __name__ == "__main__":
    main()
