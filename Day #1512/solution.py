# Day 1512: First recurring character: scan L->R, track seen set; first char already seen wins.
# O(n) time, O(alphabet) space.

def first_recurring(s):
    seen = set()
    for c in s:
        if c in seen:
            return c
        seen.add(c)
    return None


if __name__ == "__main__":
    r = first_recurring("acbbac")
    print(r if r is not None else "null")
