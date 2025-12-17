# Day 759: Generate all valid IPv4 addresses from a digit string (backtracking).
# At most 3^3 splits; Time: O(1) practically (bounded), Space: O(#results).


def valid_octet(s):
    if not s or len(s) > 3:
        return False
    if len(s) > 1 and s[0] == '0':
        return False
    return int(s) <= 255


def restore_ips(s):
    res = []

    def backtrack(start, parts):
        if len(parts) == 4:
            if start == len(s):
                res.append(".".join(parts))
            return
        for length in range(1, 4):
            if start + length > len(s):
                break
            seg = s[start:start + length]
            if valid_octet(seg):
                backtrack(start + length, parts + [seg])

    backtrack(0, [])
    return res


if __name__ == "__main__":
    print(restore_ips("2542540123"))  # ['254.25.40.123', '254.254.0.123']
