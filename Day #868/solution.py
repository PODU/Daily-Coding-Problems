# Day 868: Can any permutation of the string form a palindrome?
# Approach: count chars; palindrome possible iff at most one char has an odd count.
# Time: O(n), Space: O(alphabet).
from collections import Counter


def can_form_palindrome(s):
    odd = sum(c % 2 for c in Counter(s).values())
    return odd <= 1


if __name__ == "__main__":
    print(str(can_form_palindrome("carrace")).lower())  # true
    print(str(can_form_palindrome("daily")).lower())    # false
