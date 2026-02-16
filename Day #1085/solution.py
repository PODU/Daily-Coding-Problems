# Day 1085: Generate valid IP addresses via backtracking over 4 octets (each 0-255, no leading zeros).
# Time O(1) (bounded by string length <= 12), Space O(1) extra.
def valid(seg):
    if not seg or len(seg) > 3:
        return False
    if len(seg) > 1 and seg[0] == '0':
        return False
    return int(seg) <= 255


def restore_ip_addresses(s):
    res = []

    def backtrack(start, part, cur):
        if part == 4:
            if start == len(s):
                res.append('.'.join(cur))
            return
        for length in range(1, 4):
            if start + length > len(s):
                break
            seg = s[start:start + length]
            if valid(seg):
                backtrack(start + length, part + 1, cur + [seg])

    backtrack(0, 0, [])
    return res


if __name__ == "__main__":
    print(restore_ip_addresses("2542540123"))
