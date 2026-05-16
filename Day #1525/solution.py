# Day 1525: UTF-8 validation: read lead byte, count leading ones (1->1byte, 2..4 multi), verify continuation bytes start with 10.
# Time O(n), Space O(1).
def valid_utf8(data):
    i, n = 0, len(data)
    while i < n:
        b = data[i] & 0xFF
        if (b & 0x80) == 0x00:
            cnt = 1
        elif (b & 0xE0) == 0xC0:
            cnt = 2
        elif (b & 0xF0) == 0xE0:
            cnt = 3
        elif (b & 0xF8) == 0xF0:
            cnt = 4
        else:
            return False
        if i + cnt > n:
            return False
        for k in range(1, cnt):
            if (data[i + k] & 0xC0) != 0x80:
                return False
        i += cnt
    return True


if __name__ == "__main__":
    print(valid_utf8([226, 130, 172]))                          # True
    print(valid_utf8([0b11110101, 0b10000010, 0b00000010]))     # False
