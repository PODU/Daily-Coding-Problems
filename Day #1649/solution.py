# Day 1649: Bijective base-26: while n>0, n--, prepend 'A'+(n%26), n//=26. O(log n) time, O(log n) space.
def column_title(n: int) -> str:
    s = []
    while n > 0:
        n -= 1
        s.append(chr(ord('A') + (n % 26)))
        n //= 26
    return ''.join(reversed(s))


if __name__ == "__main__":
    print(column_title(1))
    print(column_title(27))
