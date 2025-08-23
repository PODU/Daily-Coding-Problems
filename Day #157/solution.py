# Day 157: A permutation is a palindrome iff at most one char has odd count.
# Track parity via a set of odd-count chars. Time O(n), Space O(alphabet).


def can_form_palindrome(s):
    odd = set()
    for c in s:
        if c in odd:
            odd.discard(c)
        else:
            odd.add(c)
    return len(odd) <= 1


if __name__ == "__main__":
    print(str(can_form_palindrome("carrace")).lower())  # true
    print(str(can_form_palindrome("daily")).lower())    # false
