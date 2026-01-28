# Day 976: Cryptarithmetic 3-word solver (word1 + word2 = word3) via backtracking.
# Time: O(10!/(10-k)!) for k unique letters; Space: O(k).

def solve(a, b, c):
    letters, seen = [], set()
    for w in (a, b, c):
        for ch in w:
            if ch not in seen:
                seen.add(ch)
                letters.append(ch)
    leading = {a[0], b[0], c[0]}
    assign = {}
    used = [False] * 10

    def val(w):
        v = 0
        for ch in w:
            v = v * 10 + assign[ch]
        return v

    def dfs(i):
        if i == len(letters):
            return val(a) + val(b) == val(c)
        ch = letters[i]
        for d in range(10):
            if used[d]:
                continue
            if d == 0 and ch in leading:
                continue
            used[d] = True
            assign[ch] = d
            if dfs(i + 1):
                return True
            used[d] = False
        return False

    return assign if dfs(0) else None


if __name__ == "__main__":
    m = solve("SEND", "MORE", "MONEY")
    order = "SENDMORY"
    print("{" + ", ".join("'%s': %d" % (ch, m[ch]) for ch in order) + "}")
