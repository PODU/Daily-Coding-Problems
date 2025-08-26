# Day 176: Bijective char mapping check: track s1->s2 map and set of used s2 chars; reject conflicts.
# Time O(n), Space O(unique chars).


def is_bijective(s1, s2):
    if len(s1) != len(s2):
        return False
    mapping = {}
    used = set()
    for a, b in zip(s1, s2):
        if a in mapping:
            if mapping[a] != b:
                return False
        else:
            if b in used:
                return False
            mapping[a] = b
            used.add(b)
    return True


if __name__ == "__main__":
    print(str(is_bijective("abc", "bcd")).lower())
    print(str(is_bijective("foo", "bar")).lower())
