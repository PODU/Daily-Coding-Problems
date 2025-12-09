# Day 719: Convert 1-based column number to spreadsheet id (bijective base-26).
# Repeatedly take (n-1)%26 then n=(n-1)//26. Time O(log n).

def col_id(n):
    s = []
    while n > 0:
        n -= 1
        s.append(chr(ord('A') + n % 26))
        n //= 26
    return ''.join(reversed(s))


if __name__ == "__main__":
    print('"' + col_id(1) + '"')
    print('"' + col_id(27) + '"')
