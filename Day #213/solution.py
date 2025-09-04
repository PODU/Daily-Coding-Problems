# Day 213: Generate all valid IP addresses from a digit string.
# Approach: backtracking over the 3 dot positions; each segment 1-3 digits, 0-255, no leading zeros.
# Time O(1) effectively (length <= 12), Space O(1).
from typing import List


def valid(seg: str) -> bool:
    if not seg or len(seg) > 3:
        return False
    if len(seg) > 1 and seg[0] == '0':
        return False
    return int(seg) <= 255


def restore(s: str) -> List[str]:
    res: List[str] = []

    def solve(start: int, parts: List[str]) -> None:
        if len(parts) == 4:
            if start == len(s):
                res.append(".".join(parts))
            return
        for length in range(1, 4):
            if start + length > len(s):
                break
            seg = s[start:start + length]
            if valid(seg):
                solve(start + length, parts + [seg])

    solve(0, [])
    return res


if __name__ == "__main__":
    print(restore("2542540123"))
