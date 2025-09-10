# Day 249: Maximum XOR of any two elements using a binary trie of bits.
# Insert each number, greedily pick opposite bit per number. O(n*bits) time, O(n*bits) space.

BITS = 32


def find_max_xor(nums):
    root = {}
    for x in nums:
        node = root
        for i in range(BITS - 1, -1, -1):
            b = (x >> i) & 1
            node = node.setdefault(b, {})
    best = 0
    for x in nums:
        node = root
        cur = 0
        for i in range(BITS - 1, -1, -1):
            b = (x >> i) & 1
            want = b ^ 1
            if want in node:
                cur |= (1 << i)
                node = node[want]
            else:
                node = node[b]
        best = max(best, cur)
    return best


def main():
    nums = [3, 10, 5, 25, 2, 8]
    print(find_max_xor(nums))


if __name__ == "__main__":
    main()
