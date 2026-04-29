# Day 1441: Validate UTF-8 encoding from an array of byte values.
# Approach: scan bytes, read leading-one count of lead byte, verify
# continuation bytes start with 10. Time O(n), Space O(1).
from typing import List


def valid_utf8(data: List[int]) -> bool:
    i, n = 0, len(data)
    while i < n:
        b = data[i] & 0xFF
        if b >> 7 == 0b0:
            cnt = 1
        elif b >> 5 == 0b110:
            cnt = 2
        elif b >> 4 == 0b1110:
            cnt = 3
        elif b >> 3 == 0b11110:
            cnt = 4
        else:
            return False
        if i + cnt > n:
            return False
        for j in range(1, cnt):
            if (data[i + j] & 0xFF) >> 6 != 0b10:
                return False
        i += cnt
    return True


if __name__ == "__main__":
    euro = [226, 130, 172]  # 11100010 10000010 10101100
    print("True" if valid_utf8(euro) else "False")
