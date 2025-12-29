# Day 817: Word break via DP with backpointers: dp[i] reachable, prev[i] start of last word. O(n^2) time, O(n) space.

def word_break(s, dict_words):
    n = len(s)
    prev = [-2] * (n + 1)  # -2 = unreachable
    prev[0] = -1
    for i in range(1, n + 1):
        # scan j downward -> prefer shortest last word
        for j in range(i - 1, -1, -1):
            if prev[j] != -2 and s[j:i] in dict_words:
                prev[i] = j
                break
    if prev[n] == -2:
        return None
    res = []
    i = n
    while i > 0:
        res.append(s[prev[i]:i])
        i = prev[i]
    return res[::-1]


def fmt(r):
    return "null" if r is None else "[" + ", ".join(r) + "]"


if __name__ == "__main__":
    print(fmt(word_break("thequickbrownfox", {"quick", "brown", "the", "fox"})))
    print(fmt(word_break("bedbathandbeyond", {"bed", "bath", "bedbath", "and", "beyond"})))
