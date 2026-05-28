# Day 1575: Cryptarithmetic solver via backtracking over distinct letters. O(10!/(10-k)!) worst, pruned.

def solve_crypt(w1, w2, w3):
    letters = []
    seen = set()
    for w in (w1, w2, w3):
        for c in w:
            if c not in seen:
                seen.add(c)
                letters.append(c)
    leading = {w1[0], w2[0], w3[0]}
    assign = {}
    used = [False] * 10

    def val(w):
        v = 0
        for c in w:
            v = v * 10 + assign[c]
        return v

    def bt(idx):
        if idx == len(letters):
            return val(w1) + val(w2) == val(w3)
        c = letters[idx]
        for d in range(10):
            if used[d]:
                continue
            if d == 0 and c in leading:
                continue
            used[d] = True
            assign[c] = d
            if bt(idx + 1):
                return True
            used[d] = False
        return False

    if bt(0):
        return {c: assign[c] for c in letters}
    return None


if __name__ == "__main__":
    print(solve_crypt("SEND", "MORE", "MONEY"))
