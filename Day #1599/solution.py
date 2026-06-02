# Day 1599: Generate all valid IPv4 addresses by backtracking: place 3 dots, each
# segment len 1..3, value 0..255, no leading zeros. Time O(1) (bounded).
def restore_ip(s):
    res = []

    def backtrack(start, part, cur):
        if part == 4:
            if start == len(s):
                res.append(".".join(cur))
            return
        for length in range(1, 4):
            if start + length > len(s):
                break
            seg = s[start:start + length]
            if len(seg) > 1 and seg[0] == "0":
                break
            if int(seg) > 255:
                break
            backtrack(start + length, part + 1, cur + [seg])

    backtrack(0, 0, [])
    return res


if __name__ == "__main__":
    s = "2542540123"
    print(restore_ip(s))
