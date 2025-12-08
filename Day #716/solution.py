# Day 716: Validate UTF-8. Read leading byte to get char length (1-4) from its
# high bits, then verify each continuation byte starts with 10. Time O(n).

def valid_utf8(data):
    i, n = 0, len(data)
    while i < n:
        b = data[i] & 0xFF
        if b >> 7 == 0b0:
            length = 1
        elif b >> 5 == 0b110:
            length = 2
        elif b >> 4 == 0b1110:
            length = 3
        elif b >> 3 == 0b11110:
            length = 4
        else:
            return False
        if i + length > n:
            return False
        for j in range(1, length):
            if (data[i + j] & 0xFF) >> 6 != 0b10:
                return False
        i += length
    return True


if __name__ == "__main__":
    print("True" if valid_utf8([226, 130, 172]) else "False")
    print("True" if valid_utf8([235, 140, 4]) else "False")
