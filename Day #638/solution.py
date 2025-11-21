# Day 638: Reverse the words in a string.
# Approach: split on spaces and reverse word order; in-place variant reverses
# the whole list then each word.
# Time: O(n), Space: O(n) (Python strings are immutable).
def reverse_words(s):
    return " ".join(reversed(s.split(" ")))


def reverse_words_inplace(chars):
    # chars: mutable list of characters; reverses words in O(1) extra space.
    def rev(i, j):
        while i < j:
            chars[i], chars[j] = chars[j], chars[i]
            i += 1
            j -= 1
    n = len(chars)
    rev(0, n - 1)
    i = 0
    while i < n:
        j = i
        while j < n and chars[j] != ' ':
            j += 1
        rev(i, j - 1)
        i = j + 1
    return chars


if __name__ == "__main__":
    s = "hello world here"
    assert reverse_words(s) == "".join(reverse_words_inplace(list(s)))
    print(reverse_words(s))                                # here world hello
