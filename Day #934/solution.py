# Day 934: First recurring character = first char that has been seen before while scanning.
# Hash set of seen chars; return on first repeat. Time O(n), Space O(min(n, alphabet)).


def first_recurring(s):
    seen = set()
    for c in s:
        if c in seen:
            return c
        seen.add(c)
    return None


if __name__ == "__main__":
    for s in ["acbbac", "abcdef"]:
        c = first_recurring(s)
        print(f'"{c}"' if c is not None else "null")
    # "b"
    # null
