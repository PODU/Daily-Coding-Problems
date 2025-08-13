# Day 113: Reverse word order in-place: reverse whole, then each word. O(n) time, O(1) extra.
def reverse_words(chars):
    def rev(i, j):
        while i < j:
            chars[i], chars[j] = chars[j], chars[i]
            i, j = i + 1, j - 1

    n = len(chars)
    rev(0, n - 1)
    start = 0
    for i in range(n + 1):
        if i == n or chars[i] == " ":
            rev(start, i - 1)
            start = i + 1
    return chars


if __name__ == "__main__":
    s = list("hello world here")
    print('"' + "".join(reverse_words(s)) + '"')  # "here world hello"
