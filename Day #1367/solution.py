# Day 1367: Cryptarithmetic solver via backtracking over letter->digit assignments.
# Time O(10!/(10-L)!) worst with pruning, Space O(L). L = #distinct letters.


def solve(a, b, c):
    words = [a, b, c]
    letters, seen = [], set()
    for w in words:
        for ch in w:
            if ch not in seen:
                seen.add(ch)
                letters.append(ch)
    leading = {w[0] for w in words}
    assign, used = {}, [False] * 10

    def val(w):
        v = 0
        for ch in w:
            v = v * 10 + assign[ch]
        return v

    def bt(i):
        if i == len(letters):
            return val(a) + val(b) == val(c)
        ch = letters[i]
        for d in range(10):
            if used[d] or (d == 0 and ch in leading):
                continue
            used[d] = True
            assign[ch] = d
            if bt(i + 1):
                return True
            used[d] = False
        return False

    if bt(0):
        return {ch: assign[ch] for ch in letters}
    return None


if __name__ == "__main__":
    res = solve("SEND", "MORE", "MONEY")
    print(res)
