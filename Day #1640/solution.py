# Day 1640: Greedy: track low/high possible open-paren counts in one pass.
# Time O(n), Space O(1). Balanced iff low reaches 0 at end and high never < 0.

def is_balanced(s):
    low = high = 0
    for c in s:
        if c == '(':
            low += 1; high += 1
        elif c == ')':
            low -= 1; high -= 1
        else:
            low -= 1; high += 1
        if high < 0:
            return False
        if low < 0:
            low = 0
    return low == 0


def main():
    a, b, c = "(()*", "(*)", ")*("
    ra, rb, rc = is_balanced(a), is_balanced(b), is_balanced(c)
    print("{} and {} are {}. {} is {}.".format(
        a, b, "balanced" if ra and rb else "not balanced",
        c, "balanced" if rc else "not balanced"))


if __name__ == "__main__":
    main()
