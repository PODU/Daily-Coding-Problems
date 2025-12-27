# Day 805: Spreadsheet column number -> alphabetical label (bijective base 26).
# Repeatedly take (n-1)%26 for the letter, then n=(n-1)//26. Time O(log n), Space O(log n).


def column_label(n):
    s = []
    while n > 0:
        n -= 1
        s.append(chr(ord("A") + n % 26))
        n //= 26
    return "".join(reversed(s))


if __name__ == "__main__":
    print(f'"{column_label(1)}"')   # "A"
    print(f'"{column_label(27)}"')  # "AA"
