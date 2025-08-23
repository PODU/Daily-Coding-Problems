# Day 159: First recurring character. Scan left to right tracking seen chars in
# a set; return the first already seen. Time O(n), Space O(alphabet).


def first_recurring(s):
    seen = set()
    for c in s:
        if c in seen:
            return c
        seen.add(c)
    return None


if __name__ == "__main__":
    r1 = first_recurring("acbbac")
    r2 = first_recurring("abcdef")
    print(r1 if r1 is not None else "null")  # b
    print(r2 if r2 is not None else "null")  # null
