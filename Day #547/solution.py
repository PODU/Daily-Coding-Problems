# Day 547: Maximum XOR of two elements using a binary trie, greedily pick opposite bit.
# Time O(n*32), Space O(n*32).
class Trie:
    __slots__ = ("child",)

    def __init__(self):
        self.child = [None, None]


def insert(root, num):
    cur = root
    for b in range(31, -1, -1):
        bit = (num >> b) & 1
        if cur.child[bit] is None:
            cur.child[bit] = Trie()
        cur = cur.child[bit]


def max_xor(nums):
    root = Trie()
    for x in nums:
        insert(root, x)
    best = 0
    for x in nums:
        cur = root
        cur_xor = 0
        for b in range(31, -1, -1):
            bit = (x >> b) & 1
            want = bit ^ 1
            if cur.child[want] is not None:
                cur_xor |= (1 << b)
                cur = cur.child[want]
            else:
                cur = cur.child[bit]
        best = max(best, cur_xor)
    return best


def main():
    nums = [3, 10, 5, 25, 2, 8]
    print(max_xor(nums))


if __name__ == "__main__":
    main()
