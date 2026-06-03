# Day 1607: Bijective char mapping s1->s2: lengths equal + consistent forward map + injective (no two s1 chars share an s2 char).
# Time O(n), Space O(1) (alphabet-bounded dicts).

def bijective_map(s1: str, s2: str) -> bool:
    if len(s1) != len(s2):
        return False
    fwd, rev = {}, {}
    for a, b in zip(s1, s2):
        if a in fwd:
            if fwd[a] != b:
                return False
        elif b in rev:
            return False
        else:
            fwd[a] = b
            rev[b] = a
    return True


if __name__ == "__main__":
    print(str(bijective_map("abc", "bcd")).lower())
    print(str(bijective_map("foo", "bar")).lower())
