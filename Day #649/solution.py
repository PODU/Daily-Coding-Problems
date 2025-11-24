# Day 649: First recurring character: single pass with a hash set, return first char already seen.
# Time O(n), Space O(k).
def first_recurring(s):
    seen = set()
    for c in s:
        if c in seen:
            return c
        seen.add(c)
    return None


def run(s):
    r = first_recurring(s)
    print("null" if r is None else r)


if __name__ == "__main__":
    run("acbbac")
    run("abcdef")
