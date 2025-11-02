# Day 534: Permutation-palindrome check: a permutation can be a palindrome iff at most one
# character has an odd frequency. Toggle membership in a set as we scan.
# Time: O(n); Space: O(alphabet).


def can_permute_palindrome(s):
    odd = set()
    for ch in s:
        if ch in odd:
            odd.discard(ch)
        else:
            odd.add(ch)
    return len(odd) <= 1


if __name__ == "__main__":
    print(str(can_permute_palindrome("carrace")).lower())
    print(str(can_permute_palindrome("daily")).lower())
