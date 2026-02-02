# Day 1008: Maximum XOR of two elements using a binary trie (bits high->low), greedy opposite bit.
# Time O(n*B), Space O(n*B), B = 31.

B = 31


def maximum_xor(nums):
    root = {}
    best = 0
    for x in nums:
        ins = root
        cur = root
        cur_xor = 0
        for b in range(B - 1, -1, -1):
            bit = (x >> b) & 1
            if bit not in ins:
                ins[bit] = {}
            ins = ins[bit]
            want = bit ^ 1
            if want in cur:
                cur_xor |= (1 << b)
                cur = cur[want]
            elif bit in cur:
                cur = cur[bit]
        best = max(best, cur_xor)
    return best


if __name__ == "__main__":
    nums = [3, 10, 5, 25, 2, 8]
    print(maximum_xor(nums))  # 28
