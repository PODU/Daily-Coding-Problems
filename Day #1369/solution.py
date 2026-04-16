# Day 1369: Restore IP addresses via backtracking over 4 octets. Time O(1) (<=3^3 splits),
# Space O(1) recursion depth. Each octet in [0,255], no leading zeros (except "0").


def valid(seg):
    if not seg or len(seg) > 3:
        return False
    if len(seg) > 1 and seg[0] == "0":
        return False
    return int(seg) <= 255


def restore(s):
    res = []

    def bt(start, part, cur):
        if part == 4:
            if start == len(s):
                res.append(".".join(cur))
            return
        for length in range(1, 4):
            if start + length > len(s):
                break
            seg = s[start:start + length]
            if valid(seg):
                bt(start + length, part + 1, cur + [seg])

    bt(0, 0, [])
    return res


if __name__ == "__main__":
    print(restore("2542540123"))
