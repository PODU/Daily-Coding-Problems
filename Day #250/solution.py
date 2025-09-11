# Day 250: Cryptarithmetic solver: backtracking over distinct letters with leading-zero pruning.
# Time: O(10!/(10-k)!) over k<=10 distinct letters; Space: O(k).

def solve(w1, w2, w3):
    order, seen = [], set()
    for w in (w1, w2, w3):
        for c in w:
            if c not in seen:
                seen.add(c); order.append(c)
    leading = {w1[0], w2[0], w3[0]}
    k = len(order)
    assign = {}
    used = [False] * 10

    def num(w):
        n = 0
        for c in w:
            n = n * 10 + assign[c]
        return n

    def dfs(idx):
        if idx == k:
            return num(w1) + num(w2) == num(w3)
        for d in range(10):
            if used[d]:
                continue
            if d == 0 and order[idx] in leading:
                continue
            used[d] = True; assign[order[idx]] = d
            if dfs(idx + 1):
                return True
            used[d] = False; del assign[order[idx]]
        return False

    dfs(0)
    return {c: assign[c] for c in order}


if __name__ == "__main__":
    result = solve("SEND", "MORE", "MONEY")
    print("{" + ", ".join(f"'{k}': {v}" for k, v in result.items()) + "}")
