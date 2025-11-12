# Day 584: Greedy rearrange: at each step pick highest remaining count != prev char, tie by smallest char.
# Feasible iff maxCount <= (n+1)//2. Time O(n*26), Space O(26).

def rearrange(s):
    cnt = [0] * 26
    for c in s:
        cnt[ord(c) - 97] += 1
    n = len(s)
    res = []
    prev = -1
    for _ in range(n):
        best = -1
        for i in range(26):
            if i == prev or cnt[i] <= 0:
                continue
            if best == -1 or cnt[i] > cnt[best]:
                best = i
        if best == -1:
            return None
        res.append(chr(97 + best))
        cnt[best] -= 1
        prev = best
    return "".join(res)


if __name__ == "__main__":
    a = rearrange("aaabbc")
    print(a if a is not None else "None")
    b = rearrange("aaab")
    print(b if b is not None else "None")
