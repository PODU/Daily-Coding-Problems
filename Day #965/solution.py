# Day 965: Validate whether byte values form a valid UTF-8 encoding.
# Approach: count leading 1s of lead byte, verify continuation bytes. Time O(n), Space O(1).


def valid_utf8(data):
    remaining = 0
    for b in data:
        b &= 0xFF
        if remaining == 0:
            if b >> 7 == 0:
                remaining = 0
            elif b >> 5 == 0b110:
                remaining = 1
            elif b >> 4 == 0b1110:
                remaining = 2
            elif b >> 3 == 0b11110:
                remaining = 3
            else:
                return False
        else:
            if b >> 6 != 0b10:
                return False
            remaining -= 1
    return remaining == 0


if __name__ == '__main__':
    print(valid_utf8([0b11100010, 0b10000010, 0b10101100]))  # True (Euro sign)
    print(valid_utf8([0b11110000, 0b10000000]))              # False (truncated)
