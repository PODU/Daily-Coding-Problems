# Day 1800: Palindrome integer check by reversing the number arithmetically (no string). Negatives are not palindromes. O(log10(n)).


def is_palindrome(x):
    if x < 0:
        return False
    original, reversed_num = x, 0
    while x > 0:
        reversed_num = reversed_num * 10 + x % 10
        x //= 10
    return reversed_num == original


if __name__ == "__main__":
    print(str(is_palindrome(121)).lower())  # true
