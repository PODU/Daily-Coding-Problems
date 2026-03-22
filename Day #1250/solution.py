# Day 1250: Palindrome permutation: count char parities; valid iff <=1 char has odd count. O(n) time, O(1) space.
from collections import Counter


def can_permute_palindrome(s: str) -> bool:
    return sum(v % 2 for v in Counter(s).values()) <= 1


if __name__ == "__main__":
    print(str(can_permute_palindrome("carrace")).lower())
    print(str(can_permute_palindrome("daily")).lower())
