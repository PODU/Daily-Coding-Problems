# Day 1506: Maximum XOR of two elements using a binary (bitwise prefix) trie over 32 bits.
# Insert each number's bits, query best complement. Time O(n*32), Space O(n*32).

class Trie:
    __slots__ = ("child",)
    def __init__(self):
        self.child = [None, None]


def insert_num(root, num):
    node = root
    for i in range(31, -1, -1):
        b = (num >> i) & 1
        if node.child[b] is None:
            node.child[b] = Trie()
        node = node.child[b]


def query_best(root, num):
    node = root
    best = 0
    for i in range(31, -1, -1):
        b = (num >> i) & 1
        want = b ^ 1
        if node.child[want] is not None:
            best |= (1 << i)
            node = node.child[want]
        else:
            node = node.child[b]
    return best


def find_maximum_xor(nums):
    root = Trie()
    best = 0
    for x in nums:
        insert_num(root, x)
        best = max(best, query_best(root, x))
    return best


if __name__ == "__main__":
    nums = [3, 10, 5, 25, 2, 8]
    print(find_maximum_xor(nums))
