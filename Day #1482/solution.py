# Day 1482: Integer palindrome without converting to a string.
# Reverse the number arithmetically and compare. Negatives are not palindromes.
# Time O(digits), Space O(1).

def is_palindrome(x):
    if x < 0:
        return False
    rev, n = 0, x
    while n > 0:
        rev = rev * 10 + n % 10
        n //= 10
    return rev == x


if __name__ == "__main__":
    for v in (121, 888, 678):
        print(f"{v} -> {'palindrome' if is_palindrome(v) else 'not a palindrome'}")
    # 121 -> palindrome
    # 888 -> palindrome
    # 678 -> not a palindrome
