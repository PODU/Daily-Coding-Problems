# Day 1554: Palindrome permutation: toggle chars in a set; a permutation is a palindrome
# iff at most one char has an odd count (set size <= 1). Time O(n), Space O(alphabet).


def can_form_palindrome(s):
    odd = set()
    for c in s:
        if c in odd:
            odd.discard(c)
        else:
            odd.add(c)
    return len(odd) <= 1


if __name__ == "__main__":
    s = "carrace"
    print("true" if can_form_palindrome(s) else "false")
