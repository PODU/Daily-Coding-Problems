# Day 1038: Reverse word order in-place: reverse whole char array, then reverse each word.
# Time: O(n), Space: O(1) extra (on the mutable list buffer).


def reverse_range(a, i, j):
    while i < j:
        a[i], a[j] = a[j], a[i]
        i += 1
        j -= 1


def reverse_words(a):
    reverse_range(a, 0, len(a) - 1)
    start = 0
    for i in range(len(a) + 1):
        if i == len(a) or a[i] == ' ':
            reverse_range(a, start, i - 1)
            start = i + 1


def main():
    a = list("hello world here")
    reverse_words(a)
    print('"' + "".join(a) + '"')


if __name__ == "__main__":
    main()
