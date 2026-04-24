# Day 1414: does a one-to-one (bijective) char mapping s1 -> s2 exist?
# Approach: enforce a consistent forward map AND injective (no two srcs share a target). O(n).


def can_map(s1, s2):
    if len(s1) != len(s2):
        return False
    fwd, rev = {}, {}
    for a, b in zip(s1, s2):
        if a in fwd and fwd[a] != b:
            return False
        if b in rev and rev[b] != a:
            return False
        fwd[a] = b
        rev[b] = a
    return True


if __name__ == "__main__":
    print("true" if can_map("abc", "bcd") else "false")  # true
    print("true" if can_map("foo", "bar") else "false")  # false
