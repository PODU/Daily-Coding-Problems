# Day 1183: Generate all N-digit strobogrammatic numbers (same when rotated 180).
# Recursively build from outside in using rotation pairs; drop leading zeros.
# Time O(output size), Space O(N) recursion depth.

PAIRS = [('0', '0'), ('1', '1'), ('6', '9'), ('8', '8'), ('9', '6')]
ROT = {'0': '0', '1': '1', '6': '9', '8': '8', '9': '6'}


def helper(m):
    if m == 0:
        return [""]
    if m == 1:
        return ["0", "1", "8"]
    inner = helper(m - 2)
    return [a + s + b for s in inner for a, b in PAIRS]


def strobogrammatic(n):
    out = [int(s) for s in helper(n) if not (len(s) > 1 and s[0] == '0') and s != "0"]
    return sorted(out)


def is_strobo(s):
    l, r = 0, len(s) - 1
    while l <= r:
        if s[l] not in ROT or ROT[s[l]] != s[r]:
            return False
        l += 1
        r -= 1
    return True


if __name__ == "__main__":
    print(f"2-digit strobogrammatic numbers: {strobogrammatic(2)}")
    print(f"16891 is strobogrammatic: {str(is_strobo('16891')).lower()}")
