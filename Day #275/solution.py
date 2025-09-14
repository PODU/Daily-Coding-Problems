# Day 275: Nth term of look-and-say (term 1 = "1").
# Build each term by run-length encoding the previous. Time O(N * len), Space O(len).
from itertools import groupby


def look_and_say(n):
    cur = "1"
    for _ in range(n - 1):
        cur = "".join(str(len(list(g))) + d for d, g in groupby(cur))
    return cur


if __name__ == "__main__":
    print(look_and_say(5))  # 111221
