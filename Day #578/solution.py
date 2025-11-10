# Day 578: Bijective char mapping: maintain s1->s2 (consistent) and s2->s1 (injective) maps. Time O(n), Space O(n).
def bijective(s1, s2):
    if len(s1) != len(s2):
        return False
    fwd, bwd = {}, {}
    for a, b in zip(s1, s2):
        if a in fwd and fwd[a] != b:
            return False
        if b in bwd and bwd[b] != a:
            return False
        fwd[a] = b
        bwd[b] = a
    return True


if __name__ == "__main__":
    if bijective("abc", "bcd"):
        print("true (map a to b, b to c, and c to d)")
    if not bijective("foo", "bar"):
        print("false (the o cannot map to two characters)")
