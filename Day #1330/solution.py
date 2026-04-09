# Day 1330: Column number -> spreadsheet label (bijective base-26).
# Repeatedly take (n-1)%26 for the next letter, then n=(n-1)//26. O(log n) time.

def column_title(n):
    s = []
    while n > 0:
        n -= 1
        s.append(chr(ord("A") + n % 26))
        n //= 26
    return "".join(reversed(s))


if __name__ == "__main__":
    print('"%s"' % column_title(1))   # "A"
    print('"%s"' % column_title(27))  # "AA"
