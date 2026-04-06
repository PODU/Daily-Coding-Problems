# Day 1315: Look-and-say: build each term by run-length encoding the previous one.
# Time O(sum of term lengths), Space O(length of Nth term).
from itertools import groupby


def look_and_say(n):
    s = "1"
    for _ in range(n - 1):
        s = "".join(f"{len(list(g))}{ch}" for ch, g in groupby(s))
    return s


if __name__ == "__main__":
    print(look_and_say(5))  # 111221
