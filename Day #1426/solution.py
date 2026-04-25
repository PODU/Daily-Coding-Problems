# Day 1426: Maximum XOR of any two elements in an array.
# Approach: Binary trie of bits; for each number greedily pick opposite bit.
# Time: O(n * B), Space: O(n * B) where B = number of bits.

def find_max_xor(nums):
    root = {}
    max_xor = 0
    BITS = 31
    for num in nums:
        node = root
        for b in range(BITS, -1, -1):
            bit = (num >> b) & 1
            node = node.setdefault(bit, {})
        node = root
        cur = 0
        for b in range(BITS, -1, -1):
            bit = (num >> b) & 1
            opp = 1 - bit
            if opp in node:
                cur |= (1 << b)
                node = node[opp]
            else:
                node = node[bit]
        max_xor = max(max_xor, cur)
    return max_xor


if __name__ == "__main__":
    print(find_max_xor([3, 10, 5, 25, 2, 8]))  # 28
