# Day 277: Validate UTF-8 from byte-value array. Single pass.
# Time O(N), Space O(1). Only low 8 bits of each integer are used.


def valid_utf8(data):
    remaining = 0
    for v in data:
        b = v & 0xFF
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


if __name__ == "__main__":
    print(valid_utf8([0b11100010, 0b10000010, 0b10101100]))  # True (Euro sign)
    print(valid_utf8([0b11101011, 0b10001100, 0b00000100]))  # False
