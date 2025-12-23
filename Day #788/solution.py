# Day 788: Integer palindrome check by arithmetic reversal (no string). O(log n) time, O(1) space. Negatives -> false.

def is_palindrome(n: int) -> bool:
    if n < 0:
        return False
    rev, x = 0, n
    while x > 0:
        rev = rev * 10 + x % 10
        x //= 10
    return rev == n


if __name__ == "__main__":
    print("true" if is_palindrome(121) else "false")
