# Day 212: Spreadsheet column number -> label (bijective base-26).
# Approach: repeatedly take (n-1)%26 for the digit, then n=(n-1)//26. Time O(log n), Space O(log n).
def column_label(n: int) -> str:
    s = []
    while n > 0:
        n -= 1
        s.append(chr(ord('A') + n % 26))
        n //= 26
    return "".join(reversed(s))


if __name__ == "__main__":
    print(f'"{column_label(1)}"')
    print(f'"{column_label(27)}"')
