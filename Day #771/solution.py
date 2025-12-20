# Day 771: One-to-one (bijective) character mapping between s1 and s2.
# Track forward and reverse maps; reject conflicts. O(n) time, O(1) space.


def is_one_to_one(s1, s2):
    if len(s1) != len(s2):
        return False
    fwd, rev = {}, {}
    for a, b in zip(s1, s2):
        if a in fwd and fwd[a] != b:
            return False
        if b in rev and rev[b] != a:
            return False
        fwd[a], rev[b] = b, a
    return True


if __name__ == "__main__":
    print(is_one_to_one("abc", "bcd"))  # True
    print(is_one_to_one("foo", "bar"))  # False
