# Day 202: Integer palindrome check without string conversion.
# Reverse the second half of the digits and compare with the first half.
# Time: O(log10 n), Space: O(1).


def is_palindrome(x):
    if x < 0:
        return False
    if x != 0 and x % 10 == 0:
        return False  # trailing zero, not 0 itself
    rev = 0
    while x > rev:
        rev = rev * 10 + x % 10
        x //= 10
    return x == rev or x == rev // 10


if __name__ == "__main__":
    print(str(is_palindrome(121)).lower())  # true
    print(str(is_palindrome(888)).lower())  # true
    print(str(is_palindrome(678)).lower())  # false
