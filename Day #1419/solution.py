# Day 1419: reverse the order of space-delimited words, in-place.
# Approach: reverse whole char list, then reverse each word. O(n) time, O(1) extra space (on the list).


def reverse_range(s, i, j):
    while i < j:
        s[i], s[j] = s[j], s[i]
        i += 1
        j -= 1


def reverse_words(s):
    n = len(s)
    reverse_range(s, 0, n - 1)
    start = 0
    for i in range(n + 1):
        if i == n or s[i] == " ":
            reverse_range(s, start, i - 1)
            start = i + 1
    return s


if __name__ == "__main__":
    chars = list("hello world here")
    print("".join(reverse_words(chars)))  # here world hello
